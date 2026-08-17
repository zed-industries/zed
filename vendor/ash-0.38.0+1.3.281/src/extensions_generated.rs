#![allow(unused_imports)]
use crate::vk::*;
use core::ffi::*;
#[doc = "Extensions tagged AMD"]
pub mod amd {
    #[doc = "VK_AMD_rasterization_order"]
    pub mod rasterization_order {
        use super::super::*;
        pub use {
            crate::vk::AMD_RASTERIZATION_ORDER_NAME as NAME,
            crate::vk::AMD_RASTERIZATION_ORDER_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_AMD_shader_trinary_minmax"]
    pub mod shader_trinary_minmax {
        use super::super::*;
        pub use {
            crate::vk::AMD_SHADER_TRINARY_MINMAX_NAME as NAME,
            crate::vk::AMD_SHADER_TRINARY_MINMAX_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_AMD_shader_explicit_vertex_parameter"]
    pub mod shader_explicit_vertex_parameter {
        use super::super::*;
        pub use {
            crate::vk::AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_NAME as NAME,
            crate::vk::AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_AMD_gcn_shader"]
    pub mod gcn_shader {
        use super::super::*;
        pub use {
            crate::vk::AMD_GCN_SHADER_NAME as NAME,
            crate::vk::AMD_GCN_SHADER_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_AMD_draw_indirect_count"]
    pub mod draw_indirect_count {
        use super::super::*;
        pub use {
            crate::vk::AMD_DRAW_INDIRECT_COUNT_NAME as NAME,
            crate::vk::AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_AMD_draw_indirect_count device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_AMD_draw_indirect_count device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_draw_indirect_count_amd: PFN_vkCmdDrawIndirectCount,
            pub cmd_draw_indexed_indirect_count_amd: PFN_vkCmdDrawIndexedIndirectCount,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_draw_indirect_count_amd: unsafe {
                        unsafe extern "system" fn cmd_draw_indirect_count_amd(
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
                                stringify!(cmd_draw_indirect_count_amd)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndirectCountAMD\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_draw_indirect_count_amd
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_draw_indexed_indirect_count_amd: unsafe {
                        unsafe extern "system" fn cmd_draw_indexed_indirect_count_amd(
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
                                stringify!(cmd_draw_indexed_indirect_count_amd)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdDrawIndexedIndirectCountAMD\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_draw_indexed_indirect_count_amd
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_AMD_negative_viewport_height"]
    pub mod negative_viewport_height {
        use super::super::*;
        pub use {
            crate::vk::AMD_NEGATIVE_VIEWPORT_HEIGHT_NAME as NAME,
            crate::vk::AMD_NEGATIVE_VIEWPORT_HEIGHT_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_AMD_gpu_shader_half_float"]
    pub mod gpu_shader_half_float {
        use super::super::*;
        pub use {
            crate::vk::AMD_GPU_SHADER_HALF_FLOAT_NAME as NAME,
            crate::vk::AMD_GPU_SHADER_HALF_FLOAT_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_AMD_shader_ballot"]
    pub mod shader_ballot {
        use super::super::*;
        pub use {
            crate::vk::AMD_SHADER_BALLOT_NAME as NAME,
            crate::vk::AMD_SHADER_BALLOT_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_AMD_texture_gather_bias_lod"]
    pub mod texture_gather_bias_lod {
        use super::super::*;
        pub use {
            crate::vk::AMD_TEXTURE_GATHER_BIAS_LOD_NAME as NAME,
            crate::vk::AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_AMD_shader_info"]
    pub mod shader_info {
        use super::super::*;
        pub use {
            crate::vk::AMD_SHADER_INFO_NAME as NAME,
            crate::vk::AMD_SHADER_INFO_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_AMD_shader_info device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_AMD_shader_info device-level function pointers"]
        pub struct DeviceFn {
            pub get_shader_info_amd: PFN_vkGetShaderInfoAMD,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_shader_info_amd: unsafe {
                        unsafe extern "system" fn get_shader_info_amd(
                            _device: crate::vk::Device,
                            _pipeline: Pipeline,
                            _shader_stage: ShaderStageFlags,
                            _info_type: ShaderInfoTypeAMD,
                            _p_info_size: *mut usize,
                            _p_info: *mut c_void,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(get_shader_info_amd)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetShaderInfoAMD\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_shader_info_amd
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_AMD_shader_image_load_store_lod"]
    pub mod shader_image_load_store_lod {
        use super::super::*;
        pub use {
            crate::vk::AMD_SHADER_IMAGE_LOAD_STORE_LOD_NAME as NAME,
            crate::vk::AMD_SHADER_IMAGE_LOAD_STORE_LOD_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_AMD_gpu_shader_int16"]
    pub mod gpu_shader_int16 {
        use super::super::*;
        pub use {
            crate::vk::AMD_GPU_SHADER_INT16_NAME as NAME,
            crate::vk::AMD_GPU_SHADER_INT16_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_AMD_mixed_attachment_samples"]
    pub mod mixed_attachment_samples {
        use super::super::*;
        pub use {
            crate::vk::AMD_MIXED_ATTACHMENT_SAMPLES_NAME as NAME,
            crate::vk::AMD_MIXED_ATTACHMENT_SAMPLES_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_AMD_shader_fragment_mask"]
    pub mod shader_fragment_mask {
        use super::super::*;
        pub use {
            crate::vk::AMD_SHADER_FRAGMENT_MASK_NAME as NAME,
            crate::vk::AMD_SHADER_FRAGMENT_MASK_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_AMD_buffer_marker"]
    pub mod buffer_marker {
        use super::super::*;
        pub use {
            crate::vk::AMD_BUFFER_MARKER_NAME as NAME,
            crate::vk::AMD_BUFFER_MARKER_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_AMD_buffer_marker device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_AMD_buffer_marker device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_write_buffer_marker_amd: PFN_vkCmdWriteBufferMarkerAMD,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_write_buffer_marker_amd: unsafe {
                        unsafe extern "system" fn cmd_write_buffer_marker_amd(
                            _command_buffer: CommandBuffer,
                            _pipeline_stage: PipelineStageFlags,
                            _dst_buffer: Buffer,
                            _dst_offset: DeviceSize,
                            _marker: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_write_buffer_marker_amd)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdWriteBufferMarkerAMD\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_write_buffer_marker_amd
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_AMD_pipeline_compiler_control"]
    pub mod pipeline_compiler_control {
        use super::super::*;
        pub use {
            crate::vk::AMD_PIPELINE_COMPILER_CONTROL_NAME as NAME,
            crate::vk::AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_AMD_shader_core_properties"]
    pub mod shader_core_properties {
        use super::super::*;
        pub use {
            crate::vk::AMD_SHADER_CORE_PROPERTIES_NAME as NAME,
            crate::vk::AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_AMD_memory_overallocation_behavior"]
    pub mod memory_overallocation_behavior {
        use super::super::*;
        pub use {
            crate::vk::AMD_MEMORY_OVERALLOCATION_BEHAVIOR_NAME as NAME,
            crate::vk::AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_AMD_display_native_hdr"]
    pub mod display_native_hdr {
        use super::super::*;
        pub use {
            crate::vk::AMD_DISPLAY_NATIVE_HDR_NAME as NAME,
            crate::vk::AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_AMD_display_native_hdr device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_AMD_display_native_hdr device-level function pointers"]
        pub struct DeviceFn {
            pub set_local_dimming_amd: PFN_vkSetLocalDimmingAMD,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    set_local_dimming_amd: unsafe {
                        unsafe extern "system" fn set_local_dimming_amd(
                            _device: crate::vk::Device,
                            _swap_chain: SwapchainKHR,
                            _local_dimming_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(set_local_dimming_amd)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkSetLocalDimmingAMD\0");
                        let val = _f(cname);
                        if val.is_null() {
                            set_local_dimming_amd
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_AMD_shader_core_properties2"]
    pub mod shader_core_properties2 {
        use super::super::*;
        pub use {
            crate::vk::AMD_SHADER_CORE_PROPERTIES2_NAME as NAME,
            crate::vk::AMD_SHADER_CORE_PROPERTIES2_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_AMD_device_coherent_memory"]
    pub mod device_coherent_memory {
        use super::super::*;
        pub use {
            crate::vk::AMD_DEVICE_COHERENT_MEMORY_NAME as NAME,
            crate::vk::AMD_DEVICE_COHERENT_MEMORY_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_AMD_shader_early_and_late_fragment_tests"]
    pub mod shader_early_and_late_fragment_tests {
        use super::super::*;
        pub use {
            crate::vk::AMD_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_NAME as NAME,
            crate::vk::AMD_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_SPEC_VERSION as SPEC_VERSION,
        };
    }
}
#[doc = "Extensions tagged AMDX"]
pub mod amdx {
    #[doc = "VK_AMDX_shader_enqueue"]
    pub mod shader_enqueue {
        use super::super::*;
        pub use {
            crate::vk::AMDX_SHADER_ENQUEUE_NAME as NAME,
            crate::vk::AMDX_SHADER_ENQUEUE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_AMDX_shader_enqueue device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_AMDX_shader_enqueue device-level function pointers"]
        pub struct DeviceFn {
            pub create_execution_graph_pipelines_amdx: PFN_vkCreateExecutionGraphPipelinesAMDX,
            pub get_execution_graph_pipeline_scratch_size_amdx:
                PFN_vkGetExecutionGraphPipelineScratchSizeAMDX,
            pub get_execution_graph_pipeline_node_index_amdx:
                PFN_vkGetExecutionGraphPipelineNodeIndexAMDX,
            pub cmd_initialize_graph_scratch_memory_amdx: PFN_vkCmdInitializeGraphScratchMemoryAMDX,
            pub cmd_dispatch_graph_amdx: PFN_vkCmdDispatchGraphAMDX,
            pub cmd_dispatch_graph_indirect_amdx: PFN_vkCmdDispatchGraphIndirectAMDX,
            pub cmd_dispatch_graph_indirect_count_amdx: PFN_vkCmdDispatchGraphIndirectCountAMDX,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_execution_graph_pipelines_amdx: unsafe {
                        unsafe extern "system" fn create_execution_graph_pipelines_amdx(
                            _device: crate::vk::Device,
                            _pipeline_cache: PipelineCache,
                            _create_info_count: u32,
                            _p_create_infos: *const ExecutionGraphPipelineCreateInfoAMDX<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_pipelines: *mut Pipeline,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_execution_graph_pipelines_amdx)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCreateExecutionGraphPipelinesAMDX\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            create_execution_graph_pipelines_amdx
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_execution_graph_pipeline_scratch_size_amdx: unsafe {
                        unsafe extern "system" fn get_execution_graph_pipeline_scratch_size_amdx(
                            _device: crate::vk::Device,
                            _execution_graph: Pipeline,
                            _p_size_info: *mut ExecutionGraphPipelineScratchSizeAMDX<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_execution_graph_pipeline_scratch_size_amdx)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetExecutionGraphPipelineScratchSizeAMDX\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_execution_graph_pipeline_scratch_size_amdx
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_execution_graph_pipeline_node_index_amdx: unsafe {
                        unsafe extern "system" fn get_execution_graph_pipeline_node_index_amdx(
                            _device: crate::vk::Device,
                            _execution_graph: Pipeline,
                            _p_node_info: *const PipelineShaderStageNodeCreateInfoAMDX<'_>,
                            _p_node_index: *mut u32,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_execution_graph_pipeline_node_index_amdx)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetExecutionGraphPipelineNodeIndexAMDX\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_execution_graph_pipeline_node_index_amdx
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_initialize_graph_scratch_memory_amdx: unsafe {
                        unsafe extern "system" fn cmd_initialize_graph_scratch_memory_amdx(
                            _command_buffer: CommandBuffer,
                            _scratch: DeviceAddress,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_initialize_graph_scratch_memory_amdx)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdInitializeGraphScratchMemoryAMDX\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_initialize_graph_scratch_memory_amdx
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_dispatch_graph_amdx: unsafe {
                        unsafe extern "system" fn cmd_dispatch_graph_amdx(
                            _command_buffer: CommandBuffer,
                            _scratch: DeviceAddress,
                            _p_count_info: *const DispatchGraphCountInfoAMDX,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_dispatch_graph_amdx)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdDispatchGraphAMDX\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_dispatch_graph_amdx
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_dispatch_graph_indirect_amdx: unsafe {
                        unsafe extern "system" fn cmd_dispatch_graph_indirect_amdx(
                            _command_buffer: CommandBuffer,
                            _scratch: DeviceAddress,
                            _p_count_info: *const DispatchGraphCountInfoAMDX,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_dispatch_graph_indirect_amdx)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdDispatchGraphIndirectAMDX\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_dispatch_graph_indirect_amdx
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_dispatch_graph_indirect_count_amdx: unsafe {
                        unsafe extern "system" fn cmd_dispatch_graph_indirect_count_amdx(
                            _command_buffer: CommandBuffer,
                            _scratch: DeviceAddress,
                            _count_info: DeviceAddress,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_dispatch_graph_indirect_count_amdx)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdDispatchGraphIndirectCountAMDX\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_dispatch_graph_indirect_count_amdx
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
}
#[doc = "Extensions tagged ANDROID"]
pub mod android {
    #[doc = "VK_ANDROID_native_buffer"]
    pub mod native_buffer {
        use super::super::*;
        pub use {
            crate::vk::ANDROID_NATIVE_BUFFER_NAME as NAME,
            crate::vk::ANDROID_NATIVE_BUFFER_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_ANDROID_native_buffer device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_ANDROID_native_buffer device-level function pointers"]
        pub struct DeviceFn {
            pub get_swapchain_gralloc_usage_android: PFN_vkGetSwapchainGrallocUsageANDROID,
            pub acquire_image_android: PFN_vkAcquireImageANDROID,
            pub queue_signal_release_image_android: PFN_vkQueueSignalReleaseImageANDROID,
            pub get_swapchain_gralloc_usage2_android: PFN_vkGetSwapchainGrallocUsage2ANDROID,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_swapchain_gralloc_usage_android: unsafe {
                        unsafe extern "system" fn get_swapchain_gralloc_usage_android(
                            _device: crate::vk::Device,
                            _format: Format,
                            _image_usage: ImageUsageFlags,
                            _gralloc_usage: *mut c_int,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_swapchain_gralloc_usage_android)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetSwapchainGrallocUsageANDROID\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_swapchain_gralloc_usage_android
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    acquire_image_android: unsafe {
                        unsafe extern "system" fn acquire_image_android(
                            _device: crate::vk::Device,
                            _image: Image,
                            _native_fence_fd: c_int,
                            _semaphore: Semaphore,
                            _fence: Fence,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(acquire_image_android)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkAcquireImageANDROID\0");
                        let val = _f(cname);
                        if val.is_null() {
                            acquire_image_android
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    queue_signal_release_image_android: unsafe {
                        unsafe extern "system" fn queue_signal_release_image_android(
                            _queue: Queue,
                            _wait_semaphore_count: u32,
                            _p_wait_semaphores: *const Semaphore,
                            _image: Image,
                            _p_native_fence_fd: *mut c_int,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(queue_signal_release_image_android)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkQueueSignalReleaseImageANDROID\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            queue_signal_release_image_android
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_swapchain_gralloc_usage2_android: unsafe {
                        unsafe extern "system" fn get_swapchain_gralloc_usage2_android(
                            _device: crate::vk::Device,
                            _format: Format,
                            _image_usage: ImageUsageFlags,
                            _swapchain_image_usage: SwapchainImageUsageFlagsANDROID,
                            _gralloc_consumer_usage: *mut u64,
                            _gralloc_producer_usage: *mut u64,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_swapchain_gralloc_usage2_android)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetSwapchainGrallocUsage2ANDROID\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_swapchain_gralloc_usage2_android
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_ANDROID_external_memory_android_hardware_buffer"]
    pub mod external_memory_android_hardware_buffer {
        use super::super::*;
        pub use {
            crate::vk::ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_NAME as NAME,
            crate::vk::ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_ANDROID_external_memory_android_hardware_buffer device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_ANDROID_external_memory_android_hardware_buffer device-level function pointers"]
        pub struct DeviceFn {
            pub get_android_hardware_buffer_properties_android:
                PFN_vkGetAndroidHardwareBufferPropertiesANDROID,
            pub get_memory_android_hardware_buffer_android:
                PFN_vkGetMemoryAndroidHardwareBufferANDROID,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_android_hardware_buffer_properties_android: unsafe {
                        unsafe extern "system" fn get_android_hardware_buffer_properties_android(
                            _device: crate::vk::Device,
                            _buffer: *const AHardwareBuffer,
                            _p_properties: *mut AndroidHardwareBufferPropertiesANDROID<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_android_hardware_buffer_properties_android)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetAndroidHardwareBufferPropertiesANDROID\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_android_hardware_buffer_properties_android
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_memory_android_hardware_buffer_android: unsafe {
                        unsafe extern "system" fn get_memory_android_hardware_buffer_android(
                            _device: crate::vk::Device,
                            _p_info: *const MemoryGetAndroidHardwareBufferInfoANDROID<'_>,
                            _p_buffer: *mut *mut AHardwareBuffer,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_memory_android_hardware_buffer_android)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetMemoryAndroidHardwareBufferANDROID\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_memory_android_hardware_buffer_android
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_ANDROID_external_format_resolve"]
    pub mod external_format_resolve {
        use super::super::*;
        pub use {
            crate::vk::ANDROID_EXTERNAL_FORMAT_RESOLVE_NAME as NAME,
            crate::vk::ANDROID_EXTERNAL_FORMAT_RESOLVE_SPEC_VERSION as SPEC_VERSION,
        };
    }
}
#[doc = "Extensions tagged ARM"]
pub mod arm {
    #[doc = "VK_ARM_rasterization_order_attachment_access"]
    pub mod rasterization_order_attachment_access {
        use super::super::*;
        pub use {
            crate::vk::ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_NAME as NAME,
            crate::vk::ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_ARM_shader_core_properties"]
    pub mod shader_core_properties {
        use super::super::*;
        pub use {
            crate::vk::ARM_SHADER_CORE_PROPERTIES_NAME as NAME,
            crate::vk::ARM_SHADER_CORE_PROPERTIES_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_ARM_scheduling_controls"]
    pub mod scheduling_controls {
        use super::super::*;
        pub use {
            crate::vk::ARM_SCHEDULING_CONTROLS_NAME as NAME,
            crate::vk::ARM_SCHEDULING_CONTROLS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_ARM_render_pass_striped"]
    pub mod render_pass_striped {
        use super::super::*;
        pub use {
            crate::vk::ARM_RENDER_PASS_STRIPED_NAME as NAME,
            crate::vk::ARM_RENDER_PASS_STRIPED_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_ARM_shader_core_builtins"]
    pub mod shader_core_builtins {
        use super::super::*;
        pub use {
            crate::vk::ARM_SHADER_CORE_BUILTINS_NAME as NAME,
            crate::vk::ARM_SHADER_CORE_BUILTINS_SPEC_VERSION as SPEC_VERSION,
        };
    }
}
#[doc = "Extensions tagged EXT"]
pub mod ext {
    #[doc = "VK_EXT_debug_report"]
    pub mod debug_report {
        use super::super::*;
        pub use {
            crate::vk::EXT_DEBUG_REPORT_NAME as NAME,
            crate::vk::EXT_DEBUG_REPORT_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_debug_report instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_debug_report instance-level function pointers"]
        pub struct InstanceFn {
            pub create_debug_report_callback_ext: PFN_vkCreateDebugReportCallbackEXT,
            pub destroy_debug_report_callback_ext: PFN_vkDestroyDebugReportCallbackEXT,
            pub debug_report_message_ext: PFN_vkDebugReportMessageEXT,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_debug_report_callback_ext: unsafe {
                        unsafe extern "system" fn create_debug_report_callback_ext(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const DebugReportCallbackCreateInfoEXT<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_callback: *mut DebugReportCallbackEXT,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_debug_report_callback_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCreateDebugReportCallbackEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            create_debug_report_callback_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_debug_report_callback_ext: unsafe {
                        unsafe extern "system" fn destroy_debug_report_callback_ext(
                            _instance: crate::vk::Instance,
                            _callback: DebugReportCallbackEXT,
                            _p_allocator: *const AllocationCallbacks<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(destroy_debug_report_callback_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkDestroyDebugReportCallbackEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_debug_report_callback_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    debug_report_message_ext: unsafe {
                        unsafe extern "system" fn debug_report_message_ext(
                            _instance: crate::vk::Instance,
                            _flags: DebugReportFlagsEXT,
                            _object_type: DebugReportObjectTypeEXT,
                            _object: u64,
                            _location: usize,
                            _message_code: i32,
                            _p_layer_prefix: *const c_char,
                            _p_message: *const c_char,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(debug_report_message_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkDebugReportMessageEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            debug_report_message_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_depth_range_unrestricted"]
    pub mod depth_range_unrestricted {
        use super::super::*;
        pub use {
            crate::vk::EXT_DEPTH_RANGE_UNRESTRICTED_NAME as NAME,
            crate::vk::EXT_DEPTH_RANGE_UNRESTRICTED_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_debug_marker"]
    pub mod debug_marker {
        use super::super::*;
        pub use {
            crate::vk::EXT_DEBUG_MARKER_NAME as NAME,
            crate::vk::EXT_DEBUG_MARKER_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_debug_marker device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_debug_marker device-level function pointers"]
        pub struct DeviceFn {
            pub debug_marker_set_object_tag_ext: PFN_vkDebugMarkerSetObjectTagEXT,
            pub debug_marker_set_object_name_ext: PFN_vkDebugMarkerSetObjectNameEXT,
            pub cmd_debug_marker_begin_ext: PFN_vkCmdDebugMarkerBeginEXT,
            pub cmd_debug_marker_end_ext: PFN_vkCmdDebugMarkerEndEXT,
            pub cmd_debug_marker_insert_ext: PFN_vkCmdDebugMarkerInsertEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    debug_marker_set_object_tag_ext: unsafe {
                        unsafe extern "system" fn debug_marker_set_object_tag_ext(
                            _device: crate::vk::Device,
                            _p_tag_info: *const DebugMarkerObjectTagInfoEXT<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(debug_marker_set_object_tag_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkDebugMarkerSetObjectTagEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            debug_marker_set_object_tag_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    debug_marker_set_object_name_ext: unsafe {
                        unsafe extern "system" fn debug_marker_set_object_name_ext(
                            _device: crate::vk::Device,
                            _p_name_info: *const DebugMarkerObjectNameInfoEXT<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(debug_marker_set_object_name_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkDebugMarkerSetObjectNameEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            debug_marker_set_object_name_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_debug_marker_begin_ext: unsafe {
                        unsafe extern "system" fn cmd_debug_marker_begin_ext(
                            _command_buffer: CommandBuffer,
                            _p_marker_info: *const DebugMarkerMarkerInfoEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_debug_marker_begin_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdDebugMarkerBeginEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_debug_marker_begin_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_debug_marker_end_ext: unsafe {
                        unsafe extern "system" fn cmd_debug_marker_end_ext(
                            _command_buffer: CommandBuffer,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_debug_marker_end_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdDebugMarkerEndEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_debug_marker_end_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_debug_marker_insert_ext: unsafe {
                        unsafe extern "system" fn cmd_debug_marker_insert_ext(
                            _command_buffer: CommandBuffer,
                            _p_marker_info: *const DebugMarkerMarkerInfoEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_debug_marker_insert_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdDebugMarkerInsertEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_debug_marker_insert_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_transform_feedback"]
    pub mod transform_feedback {
        use super::super::*;
        pub use {
            crate::vk::EXT_TRANSFORM_FEEDBACK_NAME as NAME,
            crate::vk::EXT_TRANSFORM_FEEDBACK_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_transform_feedback device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_transform_feedback device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_bind_transform_feedback_buffers_ext: PFN_vkCmdBindTransformFeedbackBuffersEXT,
            pub cmd_begin_transform_feedback_ext: PFN_vkCmdBeginTransformFeedbackEXT,
            pub cmd_end_transform_feedback_ext: PFN_vkCmdEndTransformFeedbackEXT,
            pub cmd_begin_query_indexed_ext: PFN_vkCmdBeginQueryIndexedEXT,
            pub cmd_end_query_indexed_ext: PFN_vkCmdEndQueryIndexedEXT,
            pub cmd_draw_indirect_byte_count_ext: PFN_vkCmdDrawIndirectByteCountEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_bind_transform_feedback_buffers_ext: unsafe {
                        unsafe extern "system" fn cmd_bind_transform_feedback_buffers_ext(
                            _command_buffer: CommandBuffer,
                            _first_binding: u32,
                            _binding_count: u32,
                            _p_buffers: *const Buffer,
                            _p_offsets: *const DeviceSize,
                            _p_sizes: *const DeviceSize,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_bind_transform_feedback_buffers_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdBindTransformFeedbackBuffersEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_bind_transform_feedback_buffers_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_begin_transform_feedback_ext: unsafe {
                        unsafe extern "system" fn cmd_begin_transform_feedback_ext(
                            _command_buffer: CommandBuffer,
                            _first_counter_buffer: u32,
                            _counter_buffer_count: u32,
                            _p_counter_buffers: *const Buffer,
                            _p_counter_buffer_offsets: *const DeviceSize,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_begin_transform_feedback_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdBeginTransformFeedbackEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_begin_transform_feedback_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_end_transform_feedback_ext: unsafe {
                        unsafe extern "system" fn cmd_end_transform_feedback_ext(
                            _command_buffer: CommandBuffer,
                            _first_counter_buffer: u32,
                            _counter_buffer_count: u32,
                            _p_counter_buffers: *const Buffer,
                            _p_counter_buffer_offsets: *const DeviceSize,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_end_transform_feedback_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdEndTransformFeedbackEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_end_transform_feedback_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_begin_query_indexed_ext: unsafe {
                        unsafe extern "system" fn cmd_begin_query_indexed_ext(
                            _command_buffer: CommandBuffer,
                            _query_pool: QueryPool,
                            _query: u32,
                            _flags: QueryControlFlags,
                            _index: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_begin_query_indexed_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginQueryIndexedEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_begin_query_indexed_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_end_query_indexed_ext: unsafe {
                        unsafe extern "system" fn cmd_end_query_indexed_ext(
                            _command_buffer: CommandBuffer,
                            _query_pool: QueryPool,
                            _query: u32,
                            _index: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_end_query_indexed_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdEndQueryIndexedEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_end_query_indexed_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_draw_indirect_byte_count_ext: unsafe {
                        unsafe extern "system" fn cmd_draw_indirect_byte_count_ext(
                            _command_buffer: CommandBuffer,
                            _instance_count: u32,
                            _first_instance: u32,
                            _counter_buffer: Buffer,
                            _counter_buffer_offset: DeviceSize,
                            _counter_offset: u32,
                            _vertex_stride: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_draw_indirect_byte_count_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndirectByteCountEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_draw_indirect_byte_count_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_validation_flags"]
    pub mod validation_flags {
        use super::super::*;
        pub use {
            crate::vk::EXT_VALIDATION_FLAGS_NAME as NAME,
            crate::vk::EXT_VALIDATION_FLAGS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_shader_subgroup_ballot"]
    pub mod shader_subgroup_ballot {
        use super::super::*;
        pub use {
            crate::vk::EXT_SHADER_SUBGROUP_BALLOT_NAME as NAME,
            crate::vk::EXT_SHADER_SUBGROUP_BALLOT_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_shader_subgroup_vote"]
    pub mod shader_subgroup_vote {
        use super::super::*;
        pub use {
            crate::vk::EXT_SHADER_SUBGROUP_VOTE_NAME as NAME,
            crate::vk::EXT_SHADER_SUBGROUP_VOTE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_texture_compression_astc_hdr"]
    pub mod texture_compression_astc_hdr {
        use super::super::*;
        pub use {
            crate::vk::EXT_TEXTURE_COMPRESSION_ASTC_HDR_NAME as NAME,
            crate::vk::EXT_TEXTURE_COMPRESSION_ASTC_HDR_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_astc_decode_mode"]
    pub mod astc_decode_mode {
        use super::super::*;
        pub use {
            crate::vk::EXT_ASTC_DECODE_MODE_NAME as NAME,
            crate::vk::EXT_ASTC_DECODE_MODE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_pipeline_robustness"]
    pub mod pipeline_robustness {
        use super::super::*;
        pub use {
            crate::vk::EXT_PIPELINE_ROBUSTNESS_NAME as NAME,
            crate::vk::EXT_PIPELINE_ROBUSTNESS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_conditional_rendering"]
    pub mod conditional_rendering {
        use super::super::*;
        pub use {
            crate::vk::EXT_CONDITIONAL_RENDERING_NAME as NAME,
            crate::vk::EXT_CONDITIONAL_RENDERING_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_conditional_rendering device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_conditional_rendering device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_begin_conditional_rendering_ext: PFN_vkCmdBeginConditionalRenderingEXT,
            pub cmd_end_conditional_rendering_ext: PFN_vkCmdEndConditionalRenderingEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_begin_conditional_rendering_ext: unsafe {
                        unsafe extern "system" fn cmd_begin_conditional_rendering_ext(
                            _command_buffer: CommandBuffer,
                            _p_conditional_rendering_begin: *const ConditionalRenderingBeginInfoEXT<
                                '_,
                            >,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_begin_conditional_rendering_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdBeginConditionalRenderingEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_begin_conditional_rendering_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_end_conditional_rendering_ext: unsafe {
                        unsafe extern "system" fn cmd_end_conditional_rendering_ext(
                            _command_buffer: CommandBuffer,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_end_conditional_rendering_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdEndConditionalRenderingEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_end_conditional_rendering_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_direct_mode_display"]
    pub mod direct_mode_display {
        use super::super::*;
        pub use {
            crate::vk::EXT_DIRECT_MODE_DISPLAY_NAME as NAME,
            crate::vk::EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_direct_mode_display instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_direct_mode_display instance-level function pointers"]
        pub struct InstanceFn {
            pub release_display_ext: PFN_vkReleaseDisplayEXT,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    release_display_ext: unsafe {
                        unsafe extern "system" fn release_display_ext(
                            _physical_device: PhysicalDevice,
                            _display: DisplayKHR,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(release_display_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkReleaseDisplayEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            release_display_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_acquire_xlib_display"]
    pub mod acquire_xlib_display {
        use super::super::*;
        pub use {
            crate::vk::EXT_ACQUIRE_XLIB_DISPLAY_NAME as NAME,
            crate::vk::EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_acquire_xlib_display instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_acquire_xlib_display instance-level function pointers"]
        pub struct InstanceFn {
            pub acquire_xlib_display_ext: PFN_vkAcquireXlibDisplayEXT,
            pub get_rand_r_output_display_ext: PFN_vkGetRandROutputDisplayEXT,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    acquire_xlib_display_ext: unsafe {
                        unsafe extern "system" fn acquire_xlib_display_ext(
                            _physical_device: PhysicalDevice,
                            _dpy: *mut Display,
                            _display: DisplayKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(acquire_xlib_display_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkAcquireXlibDisplayEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            acquire_xlib_display_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_rand_r_output_display_ext: unsafe {
                        unsafe extern "system" fn get_rand_r_output_display_ext(
                            _physical_device: PhysicalDevice,
                            _dpy: *mut Display,
                            _rr_output: RROutput,
                            _p_display: *mut DisplayKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_rand_r_output_display_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetRandROutputDisplayEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_rand_r_output_display_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_display_surface_counter"]
    pub mod display_surface_counter {
        use super::super::*;
        pub use {
            crate::vk::EXT_DISPLAY_SURFACE_COUNTER_NAME as NAME,
            crate::vk::EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_display_surface_counter instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_display_surface_counter instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_surface_capabilities2_ext:
                PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_surface_capabilities2_ext: unsafe {
                        unsafe extern "system" fn get_physical_device_surface_capabilities2_ext(
                            _physical_device: PhysicalDevice,
                            _surface: SurfaceKHR,
                            _p_surface_capabilities: *mut SurfaceCapabilities2EXT<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_surface_capabilities2_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceSurfaceCapabilities2EXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_surface_capabilities2_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_display_control"]
    pub mod display_control {
        use super::super::*;
        pub use {
            crate::vk::EXT_DISPLAY_CONTROL_NAME as NAME,
            crate::vk::EXT_DISPLAY_CONTROL_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_display_control device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_display_control device-level function pointers"]
        pub struct DeviceFn {
            pub display_power_control_ext: PFN_vkDisplayPowerControlEXT,
            pub register_device_event_ext: PFN_vkRegisterDeviceEventEXT,
            pub register_display_event_ext: PFN_vkRegisterDisplayEventEXT,
            pub get_swapchain_counter_ext: PFN_vkGetSwapchainCounterEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    display_power_control_ext: unsafe {
                        unsafe extern "system" fn display_power_control_ext(
                            _device: crate::vk::Device,
                            _display: DisplayKHR,
                            _p_display_power_info: *const DisplayPowerInfoEXT<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(display_power_control_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkDisplayPowerControlEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            display_power_control_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    register_device_event_ext: unsafe {
                        unsafe extern "system" fn register_device_event_ext(
                            _device: crate::vk::Device,
                            _p_device_event_info: *const DeviceEventInfoEXT<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_fence: *mut Fence,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(register_device_event_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkRegisterDeviceEventEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            register_device_event_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    register_display_event_ext: unsafe {
                        unsafe extern "system" fn register_display_event_ext(
                            _device: crate::vk::Device,
                            _display: DisplayKHR,
                            _p_display_event_info: *const DisplayEventInfoEXT<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_fence: *mut Fence,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(register_display_event_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkRegisterDisplayEventEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            register_display_event_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_swapchain_counter_ext: unsafe {
                        unsafe extern "system" fn get_swapchain_counter_ext(
                            _device: crate::vk::Device,
                            _swapchain: SwapchainKHR,
                            _counter: SurfaceCounterFlagsEXT,
                            _p_counter_value: *mut u64,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_swapchain_counter_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetSwapchainCounterEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_swapchain_counter_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_discard_rectangles"]
    pub mod discard_rectangles {
        use super::super::*;
        pub use {
            crate::vk::EXT_DISCARD_RECTANGLES_NAME as NAME,
            crate::vk::EXT_DISCARD_RECTANGLES_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_discard_rectangles device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_discard_rectangles device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_set_discard_rectangle_ext: PFN_vkCmdSetDiscardRectangleEXT,
            pub cmd_set_discard_rectangle_enable_ext: PFN_vkCmdSetDiscardRectangleEnableEXT,
            pub cmd_set_discard_rectangle_mode_ext: PFN_vkCmdSetDiscardRectangleModeEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_set_discard_rectangle_ext: unsafe {
                        unsafe extern "system" fn cmd_set_discard_rectangle_ext(
                            _command_buffer: CommandBuffer,
                            _first_discard_rectangle: u32,
                            _discard_rectangle_count: u32,
                            _p_discard_rectangles: *const Rect2D,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_discard_rectangle_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDiscardRectangleEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_discard_rectangle_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_discard_rectangle_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_discard_rectangle_enable_ext(
                            _command_buffer: CommandBuffer,
                            _discard_rectangle_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_discard_rectangle_enable_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetDiscardRectangleEnableEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_discard_rectangle_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_discard_rectangle_mode_ext: unsafe {
                        unsafe extern "system" fn cmd_set_discard_rectangle_mode_ext(
                            _command_buffer: CommandBuffer,
                            _discard_rectangle_mode: DiscardRectangleModeEXT,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_discard_rectangle_mode_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetDiscardRectangleModeEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_discard_rectangle_mode_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_conservative_rasterization"]
    pub mod conservative_rasterization {
        use super::super::*;
        pub use {
            crate::vk::EXT_CONSERVATIVE_RASTERIZATION_NAME as NAME,
            crate::vk::EXT_CONSERVATIVE_RASTERIZATION_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_depth_clip_enable"]
    pub mod depth_clip_enable {
        use super::super::*;
        pub use {
            crate::vk::EXT_DEPTH_CLIP_ENABLE_NAME as NAME,
            crate::vk::EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_swapchain_colorspace"]
    pub mod swapchain_colorspace {
        use super::super::*;
        pub use {
            crate::vk::EXT_SWAPCHAIN_COLORSPACE_NAME as NAME,
            crate::vk::EXT_SWAPCHAIN_COLORSPACE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_hdr_metadata"]
    pub mod hdr_metadata {
        use super::super::*;
        pub use {
            crate::vk::EXT_HDR_METADATA_NAME as NAME,
            crate::vk::EXT_HDR_METADATA_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_hdr_metadata device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_hdr_metadata device-level function pointers"]
        pub struct DeviceFn {
            pub set_hdr_metadata_ext: PFN_vkSetHdrMetadataEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    set_hdr_metadata_ext: unsafe {
                        unsafe extern "system" fn set_hdr_metadata_ext(
                            _device: crate::vk::Device,
                            _swapchain_count: u32,
                            _p_swapchains: *const SwapchainKHR,
                            _p_metadata: *const HdrMetadataEXT<'_>,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(set_hdr_metadata_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkSetHdrMetadataEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            set_hdr_metadata_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_external_memory_dma_buf"]
    pub mod external_memory_dma_buf {
        use super::super::*;
        pub use {
            crate::vk::EXT_EXTERNAL_MEMORY_DMA_BUF_NAME as NAME,
            crate::vk::EXT_EXTERNAL_MEMORY_DMA_BUF_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_queue_family_foreign"]
    pub mod queue_family_foreign {
        use super::super::*;
        pub use {
            crate::vk::EXT_QUEUE_FAMILY_FOREIGN_NAME as NAME,
            crate::vk::EXT_QUEUE_FAMILY_FOREIGN_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_debug_utils"]
    pub mod debug_utils {
        use super::super::*;
        pub use {
            crate::vk::EXT_DEBUG_UTILS_NAME as NAME,
            crate::vk::EXT_DEBUG_UTILS_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_debug_utils instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_debug_utils instance-level function pointers"]
        pub struct InstanceFn {
            pub create_debug_utils_messenger_ext: PFN_vkCreateDebugUtilsMessengerEXT,
            pub destroy_debug_utils_messenger_ext: PFN_vkDestroyDebugUtilsMessengerEXT,
            pub submit_debug_utils_message_ext: PFN_vkSubmitDebugUtilsMessageEXT,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_debug_utils_messenger_ext: unsafe {
                        unsafe extern "system" fn create_debug_utils_messenger_ext(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const DebugUtilsMessengerCreateInfoEXT<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_messenger: *mut DebugUtilsMessengerEXT,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_debug_utils_messenger_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCreateDebugUtilsMessengerEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            create_debug_utils_messenger_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_debug_utils_messenger_ext: unsafe {
                        unsafe extern "system" fn destroy_debug_utils_messenger_ext(
                            _instance: crate::vk::Instance,
                            _messenger: DebugUtilsMessengerEXT,
                            _p_allocator: *const AllocationCallbacks<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(destroy_debug_utils_messenger_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkDestroyDebugUtilsMessengerEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_debug_utils_messenger_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    submit_debug_utils_message_ext: unsafe {
                        unsafe extern "system" fn submit_debug_utils_message_ext(
                            _instance: crate::vk::Instance,
                            _message_severity: DebugUtilsMessageSeverityFlagsEXT,
                            _message_types: DebugUtilsMessageTypeFlagsEXT,
                            _p_callback_data: *const DebugUtilsMessengerCallbackDataEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(submit_debug_utils_message_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkSubmitDebugUtilsMessageEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            submit_debug_utils_message_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
        #[doc = "VK_EXT_debug_utils device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_debug_utils device-level function pointers"]
        pub struct DeviceFn {
            pub set_debug_utils_object_name_ext: PFN_vkSetDebugUtilsObjectNameEXT,
            pub set_debug_utils_object_tag_ext: PFN_vkSetDebugUtilsObjectTagEXT,
            pub queue_begin_debug_utils_label_ext: PFN_vkQueueBeginDebugUtilsLabelEXT,
            pub queue_end_debug_utils_label_ext: PFN_vkQueueEndDebugUtilsLabelEXT,
            pub queue_insert_debug_utils_label_ext: PFN_vkQueueInsertDebugUtilsLabelEXT,
            pub cmd_begin_debug_utils_label_ext: PFN_vkCmdBeginDebugUtilsLabelEXT,
            pub cmd_end_debug_utils_label_ext: PFN_vkCmdEndDebugUtilsLabelEXT,
            pub cmd_insert_debug_utils_label_ext: PFN_vkCmdInsertDebugUtilsLabelEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    set_debug_utils_object_name_ext: unsafe {
                        unsafe extern "system" fn set_debug_utils_object_name_ext(
                            _device: crate::vk::Device,
                            _p_name_info: *const DebugUtilsObjectNameInfoEXT<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(set_debug_utils_object_name_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkSetDebugUtilsObjectNameEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            set_debug_utils_object_name_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    set_debug_utils_object_tag_ext: unsafe {
                        unsafe extern "system" fn set_debug_utils_object_tag_ext(
                            _device: crate::vk::Device,
                            _p_tag_info: *const DebugUtilsObjectTagInfoEXT<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(set_debug_utils_object_tag_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkSetDebugUtilsObjectTagEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            set_debug_utils_object_tag_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    queue_begin_debug_utils_label_ext: unsafe {
                        unsafe extern "system" fn queue_begin_debug_utils_label_ext(
                            _queue: Queue,
                            _p_label_info: *const DebugUtilsLabelEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(queue_begin_debug_utils_label_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkQueueBeginDebugUtilsLabelEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            queue_begin_debug_utils_label_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    queue_end_debug_utils_label_ext: unsafe {
                        unsafe extern "system" fn queue_end_debug_utils_label_ext(_queue: Queue) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(queue_end_debug_utils_label_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkQueueEndDebugUtilsLabelEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            queue_end_debug_utils_label_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    queue_insert_debug_utils_label_ext: unsafe {
                        unsafe extern "system" fn queue_insert_debug_utils_label_ext(
                            _queue: Queue,
                            _p_label_info: *const DebugUtilsLabelEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(queue_insert_debug_utils_label_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkQueueInsertDebugUtilsLabelEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            queue_insert_debug_utils_label_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_begin_debug_utils_label_ext: unsafe {
                        unsafe extern "system" fn cmd_begin_debug_utils_label_ext(
                            _command_buffer: CommandBuffer,
                            _p_label_info: *const DebugUtilsLabelEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_begin_debug_utils_label_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginDebugUtilsLabelEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_begin_debug_utils_label_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_end_debug_utils_label_ext: unsafe {
                        unsafe extern "system" fn cmd_end_debug_utils_label_ext(
                            _command_buffer: CommandBuffer,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_end_debug_utils_label_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdEndDebugUtilsLabelEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_end_debug_utils_label_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_insert_debug_utils_label_ext: unsafe {
                        unsafe extern "system" fn cmd_insert_debug_utils_label_ext(
                            _command_buffer: CommandBuffer,
                            _p_label_info: *const DebugUtilsLabelEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_insert_debug_utils_label_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdInsertDebugUtilsLabelEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_insert_debug_utils_label_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_sampler_filter_minmax"]
    pub mod sampler_filter_minmax {
        use super::super::*;
        pub use {
            crate::vk::EXT_SAMPLER_FILTER_MINMAX_NAME as NAME,
            crate::vk::EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_inline_uniform_block"]
    pub mod inline_uniform_block {
        use super::super::*;
        pub use {
            crate::vk::EXT_INLINE_UNIFORM_BLOCK_NAME as NAME,
            crate::vk::EXT_INLINE_UNIFORM_BLOCK_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_shader_stencil_export"]
    pub mod shader_stencil_export {
        use super::super::*;
        pub use {
            crate::vk::EXT_SHADER_STENCIL_EXPORT_NAME as NAME,
            crate::vk::EXT_SHADER_STENCIL_EXPORT_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_sample_locations"]
    pub mod sample_locations {
        use super::super::*;
        pub use {
            crate::vk::EXT_SAMPLE_LOCATIONS_NAME as NAME,
            crate::vk::EXT_SAMPLE_LOCATIONS_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_sample_locations instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_sample_locations instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_multisample_properties_ext:
                PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_multisample_properties_ext: unsafe {
                        unsafe extern "system" fn get_physical_device_multisample_properties_ext(
                            _physical_device: PhysicalDevice,
                            _samples: SampleCountFlags,
                            _p_multisample_properties: *mut MultisamplePropertiesEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_multisample_properties_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceMultisamplePropertiesEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_multisample_properties_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
        #[doc = "VK_EXT_sample_locations device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_sample_locations device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_set_sample_locations_ext: PFN_vkCmdSetSampleLocationsEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_set_sample_locations_ext: unsafe {
                        unsafe extern "system" fn cmd_set_sample_locations_ext(
                            _command_buffer: CommandBuffer,
                            _p_sample_locations_info: *const SampleLocationsInfoEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_sample_locations_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetSampleLocationsEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_sample_locations_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_blend_operation_advanced"]
    pub mod blend_operation_advanced {
        use super::super::*;
        pub use {
            crate::vk::EXT_BLEND_OPERATION_ADVANCED_NAME as NAME,
            crate::vk::EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_post_depth_coverage"]
    pub mod post_depth_coverage {
        use super::super::*;
        pub use {
            crate::vk::EXT_POST_DEPTH_COVERAGE_NAME as NAME,
            crate::vk::EXT_POST_DEPTH_COVERAGE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_image_drm_format_modifier"]
    pub mod image_drm_format_modifier {
        use super::super::*;
        pub use {
            crate::vk::EXT_IMAGE_DRM_FORMAT_MODIFIER_NAME as NAME,
            crate::vk::EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_image_drm_format_modifier device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_image_drm_format_modifier device-level function pointers"]
        pub struct DeviceFn {
            pub get_image_drm_format_modifier_properties_ext:
                PFN_vkGetImageDrmFormatModifierPropertiesEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_image_drm_format_modifier_properties_ext: unsafe {
                        unsafe extern "system" fn get_image_drm_format_modifier_properties_ext(
                            _device: crate::vk::Device,
                            _image: Image,
                            _p_properties: *mut ImageDrmFormatModifierPropertiesEXT<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_image_drm_format_modifier_properties_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetImageDrmFormatModifierPropertiesEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_image_drm_format_modifier_properties_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_validation_cache"]
    pub mod validation_cache {
        use super::super::*;
        pub use {
            crate::vk::EXT_VALIDATION_CACHE_NAME as NAME,
            crate::vk::EXT_VALIDATION_CACHE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_validation_cache device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_validation_cache device-level function pointers"]
        pub struct DeviceFn {
            pub create_validation_cache_ext: PFN_vkCreateValidationCacheEXT,
            pub destroy_validation_cache_ext: PFN_vkDestroyValidationCacheEXT,
            pub merge_validation_caches_ext: PFN_vkMergeValidationCachesEXT,
            pub get_validation_cache_data_ext: PFN_vkGetValidationCacheDataEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_validation_cache_ext: unsafe {
                        unsafe extern "system" fn create_validation_cache_ext(
                            _device: crate::vk::Device,
                            _p_create_info: *const ValidationCacheCreateInfoEXT<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_validation_cache: *mut ValidationCacheEXT,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_validation_cache_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateValidationCacheEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_validation_cache_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_validation_cache_ext: unsafe {
                        unsafe extern "system" fn destroy_validation_cache_ext(
                            _device: crate::vk::Device,
                            _validation_cache: ValidationCacheEXT,
                            _p_allocator: *const AllocationCallbacks<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(destroy_validation_cache_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkDestroyValidationCacheEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_validation_cache_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    merge_validation_caches_ext: unsafe {
                        unsafe extern "system" fn merge_validation_caches_ext(
                            _device: crate::vk::Device,
                            _dst_cache: ValidationCacheEXT,
                            _src_cache_count: u32,
                            _p_src_caches: *const ValidationCacheEXT,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(merge_validation_caches_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkMergeValidationCachesEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            merge_validation_caches_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_validation_cache_data_ext: unsafe {
                        unsafe extern "system" fn get_validation_cache_data_ext(
                            _device: crate::vk::Device,
                            _validation_cache: ValidationCacheEXT,
                            _p_data_size: *mut usize,
                            _p_data: *mut c_void,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_validation_cache_data_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetValidationCacheDataEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_validation_cache_data_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_descriptor_indexing"]
    pub mod descriptor_indexing {
        use super::super::*;
        pub use {
            crate::vk::EXT_DESCRIPTOR_INDEXING_NAME as NAME,
            crate::vk::EXT_DESCRIPTOR_INDEXING_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_shader_viewport_index_layer"]
    pub mod shader_viewport_index_layer {
        use super::super::*;
        pub use {
            crate::vk::EXT_SHADER_VIEWPORT_INDEX_LAYER_NAME as NAME,
            crate::vk::EXT_SHADER_VIEWPORT_INDEX_LAYER_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_filter_cubic"]
    pub mod filter_cubic {
        use super::super::*;
        pub use {
            crate::vk::EXT_FILTER_CUBIC_NAME as NAME,
            crate::vk::EXT_FILTER_CUBIC_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_global_priority"]
    pub mod global_priority {
        use super::super::*;
        pub use {
            crate::vk::EXT_GLOBAL_PRIORITY_NAME as NAME,
            crate::vk::EXT_GLOBAL_PRIORITY_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_external_memory_host"]
    pub mod external_memory_host {
        use super::super::*;
        pub use {
            crate::vk::EXT_EXTERNAL_MEMORY_HOST_NAME as NAME,
            crate::vk::EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_external_memory_host device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_external_memory_host device-level function pointers"]
        pub struct DeviceFn {
            pub get_memory_host_pointer_properties_ext: PFN_vkGetMemoryHostPointerPropertiesEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_memory_host_pointer_properties_ext: unsafe {
                        unsafe extern "system" fn get_memory_host_pointer_properties_ext(
                            _device: crate::vk::Device,
                            _handle_type: ExternalMemoryHandleTypeFlags,
                            _p_host_pointer: *const c_void,
                            _p_memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT<
                                '_,
                            >,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_memory_host_pointer_properties_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetMemoryHostPointerPropertiesEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_memory_host_pointer_properties_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_calibrated_timestamps"]
    pub mod calibrated_timestamps {
        use super::super::*;
        pub use {
            crate::vk::EXT_CALIBRATED_TIMESTAMPS_NAME as NAME,
            crate::vk::EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_calibrated_timestamps instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_calibrated_timestamps instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_calibrateable_time_domains_ext:
                PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_calibrateable_time_domains_ext: unsafe {
                        unsafe extern "system" fn get_physical_device_calibrateable_time_domains_ext(
                            _physical_device: PhysicalDevice,
                            _p_time_domain_count: *mut u32,
                            _p_time_domains: *mut TimeDomainKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_calibrateable_time_domains_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceCalibrateableTimeDomainsEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_calibrateable_time_domains_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
        #[doc = "VK_EXT_calibrated_timestamps device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_calibrated_timestamps device-level function pointers"]
        pub struct DeviceFn {
            pub get_calibrated_timestamps_ext: PFN_vkGetCalibratedTimestampsKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_calibrated_timestamps_ext: unsafe {
                        unsafe extern "system" fn get_calibrated_timestamps_ext(
                            _device: crate::vk::Device,
                            _timestamp_count: u32,
                            _p_timestamp_infos: *const CalibratedTimestampInfoKHR<'_>,
                            _p_timestamps: *mut u64,
                            _p_max_deviation: *mut u64,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_calibrated_timestamps_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetCalibratedTimestampsEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_calibrated_timestamps_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_vertex_attribute_divisor"]
    pub mod vertex_attribute_divisor {
        use super::super::*;
        pub use {
            crate::vk::EXT_VERTEX_ATTRIBUTE_DIVISOR_NAME as NAME,
            crate::vk::EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_pipeline_creation_feedback"]
    pub mod pipeline_creation_feedback {
        use super::super::*;
        pub use {
            crate::vk::EXT_PIPELINE_CREATION_FEEDBACK_NAME as NAME,
            crate::vk::EXT_PIPELINE_CREATION_FEEDBACK_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_pci_bus_info"]
    pub mod pci_bus_info {
        use super::super::*;
        pub use {
            crate::vk::EXT_PCI_BUS_INFO_NAME as NAME,
            crate::vk::EXT_PCI_BUS_INFO_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_metal_surface"]
    pub mod metal_surface {
        use super::super::*;
        pub use {
            crate::vk::EXT_METAL_SURFACE_NAME as NAME,
            crate::vk::EXT_METAL_SURFACE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_metal_surface instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_metal_surface instance-level function pointers"]
        pub struct InstanceFn {
            pub create_metal_surface_ext: PFN_vkCreateMetalSurfaceEXT,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_metal_surface_ext: unsafe {
                        unsafe extern "system" fn create_metal_surface_ext(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const MetalSurfaceCreateInfoEXT<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_surface: *mut SurfaceKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_metal_surface_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateMetalSurfaceEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_metal_surface_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_fragment_density_map"]
    pub mod fragment_density_map {
        use super::super::*;
        pub use {
            crate::vk::EXT_FRAGMENT_DENSITY_MAP_NAME as NAME,
            crate::vk::EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_scalar_block_layout"]
    pub mod scalar_block_layout {
        use super::super::*;
        pub use {
            crate::vk::EXT_SCALAR_BLOCK_LAYOUT_NAME as NAME,
            crate::vk::EXT_SCALAR_BLOCK_LAYOUT_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_subgroup_size_control"]
    pub mod subgroup_size_control {
        use super::super::*;
        pub use {
            crate::vk::EXT_SUBGROUP_SIZE_CONTROL_NAME as NAME,
            crate::vk::EXT_SUBGROUP_SIZE_CONTROL_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_shader_image_atomic_int64"]
    pub mod shader_image_atomic_int64 {
        use super::super::*;
        pub use {
            crate::vk::EXT_SHADER_IMAGE_ATOMIC_INT64_NAME as NAME,
            crate::vk::EXT_SHADER_IMAGE_ATOMIC_INT64_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_memory_budget"]
    pub mod memory_budget {
        use super::super::*;
        pub use {
            crate::vk::EXT_MEMORY_BUDGET_NAME as NAME,
            crate::vk::EXT_MEMORY_BUDGET_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_memory_priority"]
    pub mod memory_priority {
        use super::super::*;
        pub use {
            crate::vk::EXT_MEMORY_PRIORITY_NAME as NAME,
            crate::vk::EXT_MEMORY_PRIORITY_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_buffer_device_address"]
    pub mod buffer_device_address {
        use super::super::*;
        pub use {
            crate::vk::EXT_BUFFER_DEVICE_ADDRESS_NAME as NAME,
            crate::vk::EXT_BUFFER_DEVICE_ADDRESS_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_buffer_device_address device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_buffer_device_address device-level function pointers"]
        pub struct DeviceFn {
            pub get_buffer_device_address_ext: PFN_vkGetBufferDeviceAddress,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_buffer_device_address_ext: unsafe {
                        unsafe extern "system" fn get_buffer_device_address_ext(
                            _device: crate::vk::Device,
                            _p_info: *const BufferDeviceAddressInfo<'_>,
                        ) -> DeviceAddress {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_buffer_device_address_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetBufferDeviceAddressEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_buffer_device_address_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_tooling_info"]
    pub mod tooling_info {
        use super::super::*;
        pub use {
            crate::vk::EXT_TOOLING_INFO_NAME as NAME,
            crate::vk::EXT_TOOLING_INFO_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_tooling_info instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_tooling_info instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_tool_properties_ext: PFN_vkGetPhysicalDeviceToolProperties,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_tool_properties_ext: unsafe {
                        unsafe extern "system" fn get_physical_device_tool_properties_ext(
                            _physical_device: PhysicalDevice,
                            _p_tool_count: *mut u32,
                            _p_tool_properties: *mut PhysicalDeviceToolProperties<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_tool_properties_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceToolPropertiesEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_tool_properties_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_separate_stencil_usage"]
    pub mod separate_stencil_usage {
        use super::super::*;
        pub use {
            crate::vk::EXT_SEPARATE_STENCIL_USAGE_NAME as NAME,
            crate::vk::EXT_SEPARATE_STENCIL_USAGE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_validation_features"]
    pub mod validation_features {
        use super::super::*;
        pub use {
            crate::vk::EXT_VALIDATION_FEATURES_NAME as NAME,
            crate::vk::EXT_VALIDATION_FEATURES_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_fragment_shader_interlock"]
    pub mod fragment_shader_interlock {
        use super::super::*;
        pub use {
            crate::vk::EXT_FRAGMENT_SHADER_INTERLOCK_NAME as NAME,
            crate::vk::EXT_FRAGMENT_SHADER_INTERLOCK_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_ycbcr_image_arrays"]
    pub mod ycbcr_image_arrays {
        use super::super::*;
        pub use {
            crate::vk::EXT_YCBCR_IMAGE_ARRAYS_NAME as NAME,
            crate::vk::EXT_YCBCR_IMAGE_ARRAYS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_provoking_vertex"]
    pub mod provoking_vertex {
        use super::super::*;
        pub use {
            crate::vk::EXT_PROVOKING_VERTEX_NAME as NAME,
            crate::vk::EXT_PROVOKING_VERTEX_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_full_screen_exclusive"]
    pub mod full_screen_exclusive {
        use super::super::*;
        pub use {
            crate::vk::EXT_FULL_SCREEN_EXCLUSIVE_NAME as NAME,
            crate::vk::EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_full_screen_exclusive instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_full_screen_exclusive instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_surface_present_modes2_ext:
                PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_surface_present_modes2_ext: unsafe {
                        unsafe extern "system" fn get_physical_device_surface_present_modes2_ext(
                            _physical_device: PhysicalDevice,
                            _p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
                            _p_present_mode_count: *mut u32,
                            _p_present_modes: *mut PresentModeKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_surface_present_modes2_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceSurfacePresentModes2EXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_surface_present_modes2_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
        #[doc = "VK_EXT_full_screen_exclusive device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_full_screen_exclusive device-level function pointers"]
        pub struct DeviceFn {
            pub acquire_full_screen_exclusive_mode_ext: PFN_vkAcquireFullScreenExclusiveModeEXT,
            pub release_full_screen_exclusive_mode_ext: PFN_vkReleaseFullScreenExclusiveModeEXT,
            pub get_device_group_surface_present_modes2_ext:
                PFN_vkGetDeviceGroupSurfacePresentModes2EXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    acquire_full_screen_exclusive_mode_ext: unsafe {
                        unsafe extern "system" fn acquire_full_screen_exclusive_mode_ext(
                            _device: crate::vk::Device,
                            _swapchain: SwapchainKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(acquire_full_screen_exclusive_mode_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkAcquireFullScreenExclusiveModeEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            acquire_full_screen_exclusive_mode_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    release_full_screen_exclusive_mode_ext: unsafe {
                        unsafe extern "system" fn release_full_screen_exclusive_mode_ext(
                            _device: crate::vk::Device,
                            _swapchain: SwapchainKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(release_full_screen_exclusive_mode_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkReleaseFullScreenExclusiveModeEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            release_full_screen_exclusive_mode_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_device_group_surface_present_modes2_ext: unsafe {
                        unsafe extern "system" fn get_device_group_surface_present_modes2_ext(
                            _device: crate::vk::Device,
                            _p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
                            _p_modes: *mut DeviceGroupPresentModeFlagsKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_device_group_surface_present_modes2_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeviceGroupSurfacePresentModes2EXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_device_group_surface_present_modes2_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_headless_surface"]
    pub mod headless_surface {
        use super::super::*;
        pub use {
            crate::vk::EXT_HEADLESS_SURFACE_NAME as NAME,
            crate::vk::EXT_HEADLESS_SURFACE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_headless_surface instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_headless_surface instance-level function pointers"]
        pub struct InstanceFn {
            pub create_headless_surface_ext: PFN_vkCreateHeadlessSurfaceEXT,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_headless_surface_ext: unsafe {
                        unsafe extern "system" fn create_headless_surface_ext(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const HeadlessSurfaceCreateInfoEXT<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_surface: *mut SurfaceKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_headless_surface_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateHeadlessSurfaceEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_headless_surface_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_line_rasterization"]
    pub mod line_rasterization {
        use super::super::*;
        pub use {
            crate::vk::EXT_LINE_RASTERIZATION_NAME as NAME,
            crate::vk::EXT_LINE_RASTERIZATION_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_line_rasterization device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_line_rasterization device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_set_line_stipple_ext: PFN_vkCmdSetLineStippleKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_set_line_stipple_ext: unsafe {
                        unsafe extern "system" fn cmd_set_line_stipple_ext(
                            _command_buffer: CommandBuffer,
                            _line_stipple_factor: u32,
                            _line_stipple_pattern: u16,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_line_stipple_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLineStippleEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_line_stipple_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_shader_atomic_float"]
    pub mod shader_atomic_float {
        use super::super::*;
        pub use {
            crate::vk::EXT_SHADER_ATOMIC_FLOAT_NAME as NAME,
            crate::vk::EXT_SHADER_ATOMIC_FLOAT_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_host_query_reset"]
    pub mod host_query_reset {
        use super::super::*;
        pub use {
            crate::vk::EXT_HOST_QUERY_RESET_NAME as NAME,
            crate::vk::EXT_HOST_QUERY_RESET_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_host_query_reset device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_host_query_reset device-level function pointers"]
        pub struct DeviceFn {
            pub reset_query_pool_ext: PFN_vkResetQueryPool,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    reset_query_pool_ext: unsafe {
                        unsafe extern "system" fn reset_query_pool_ext(
                            _device: crate::vk::Device,
                            _query_pool: QueryPool,
                            _first_query: u32,
                            _query_count: u32,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(reset_query_pool_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkResetQueryPoolEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            reset_query_pool_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_index_type_uint8"]
    pub mod index_type_uint8 {
        use super::super::*;
        pub use {
            crate::vk::EXT_INDEX_TYPE_UINT8_NAME as NAME,
            crate::vk::EXT_INDEX_TYPE_UINT8_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_extended_dynamic_state"]
    pub mod extended_dynamic_state {
        use super::super::*;
        pub use {
            crate::vk::EXT_EXTENDED_DYNAMIC_STATE_NAME as NAME,
            crate::vk::EXT_EXTENDED_DYNAMIC_STATE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_extended_dynamic_state device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_extended_dynamic_state device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_set_cull_mode_ext: PFN_vkCmdSetCullMode,
            pub cmd_set_front_face_ext: PFN_vkCmdSetFrontFace,
            pub cmd_set_primitive_topology_ext: PFN_vkCmdSetPrimitiveTopology,
            pub cmd_set_viewport_with_count_ext: PFN_vkCmdSetViewportWithCount,
            pub cmd_set_scissor_with_count_ext: PFN_vkCmdSetScissorWithCount,
            pub cmd_bind_vertex_buffers2_ext: PFN_vkCmdBindVertexBuffers2,
            pub cmd_set_depth_test_enable_ext: PFN_vkCmdSetDepthTestEnable,
            pub cmd_set_depth_write_enable_ext: PFN_vkCmdSetDepthWriteEnable,
            pub cmd_set_depth_compare_op_ext: PFN_vkCmdSetDepthCompareOp,
            pub cmd_set_depth_bounds_test_enable_ext: PFN_vkCmdSetDepthBoundsTestEnable,
            pub cmd_set_stencil_test_enable_ext: PFN_vkCmdSetStencilTestEnable,
            pub cmd_set_stencil_op_ext: PFN_vkCmdSetStencilOp,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_set_cull_mode_ext: unsafe {
                        unsafe extern "system" fn cmd_set_cull_mode_ext(
                            _command_buffer: CommandBuffer,
                            _cull_mode: CullModeFlags,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_cull_mode_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetCullModeEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_cull_mode_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_front_face_ext: unsafe {
                        unsafe extern "system" fn cmd_set_front_face_ext(
                            _command_buffer: CommandBuffer,
                            _front_face: FrontFace,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_front_face_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetFrontFaceEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_front_face_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_primitive_topology_ext: unsafe {
                        unsafe extern "system" fn cmd_set_primitive_topology_ext(
                            _command_buffer: CommandBuffer,
                            _primitive_topology: PrimitiveTopology,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_primitive_topology_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetPrimitiveTopologyEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_primitive_topology_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_viewport_with_count_ext: unsafe {
                        unsafe extern "system" fn cmd_set_viewport_with_count_ext(
                            _command_buffer: CommandBuffer,
                            _viewport_count: u32,
                            _p_viewports: *const Viewport,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_viewport_with_count_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetViewportWithCountEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_viewport_with_count_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_scissor_with_count_ext: unsafe {
                        unsafe extern "system" fn cmd_set_scissor_with_count_ext(
                            _command_buffer: CommandBuffer,
                            _scissor_count: u32,
                            _p_scissors: *const Rect2D,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_scissor_with_count_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetScissorWithCountEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_scissor_with_count_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_bind_vertex_buffers2_ext: unsafe {
                        unsafe extern "system" fn cmd_bind_vertex_buffers2_ext(
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
                                stringify!(cmd_bind_vertex_buffers2_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdBindVertexBuffers2EXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_bind_vertex_buffers2_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_test_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_test_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_test_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_test_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthTestEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_test_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_write_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_write_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_write_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_write_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthWriteEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_write_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_compare_op_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_compare_op_ext(
                            _command_buffer: CommandBuffer,
                            _depth_compare_op: CompareOp,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_compare_op_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthCompareOpEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_compare_op_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_bounds_test_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_bounds_test_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_bounds_test_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_bounds_test_enable_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetDepthBoundsTestEnableEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_bounds_test_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_stencil_test_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_stencil_test_enable_ext(
                            _command_buffer: CommandBuffer,
                            _stencil_test_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_stencil_test_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilTestEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_stencil_test_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_stencil_op_ext: unsafe {
                        unsafe extern "system" fn cmd_set_stencil_op_ext(
                            _command_buffer: CommandBuffer,
                            _face_mask: StencilFaceFlags,
                            _fail_op: StencilOp,
                            _pass_op: StencilOp,
                            _depth_fail_op: StencilOp,
                            _compare_op: CompareOp,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_stencil_op_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilOpEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_stencil_op_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_host_image_copy"]
    pub mod host_image_copy {
        use super::super::*;
        pub use {
            crate::vk::EXT_HOST_IMAGE_COPY_NAME as NAME,
            crate::vk::EXT_HOST_IMAGE_COPY_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_host_image_copy device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_host_image_copy device-level function pointers"]
        pub struct DeviceFn {
            pub copy_memory_to_image_ext: PFN_vkCopyMemoryToImageEXT,
            pub copy_image_to_memory_ext: PFN_vkCopyImageToMemoryEXT,
            pub copy_image_to_image_ext: PFN_vkCopyImageToImageEXT,
            pub transition_image_layout_ext: PFN_vkTransitionImageLayoutEXT,
            pub get_image_subresource_layout2_ext: PFN_vkGetImageSubresourceLayout2KHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    copy_memory_to_image_ext: unsafe {
                        unsafe extern "system" fn copy_memory_to_image_ext(
                            _device: crate::vk::Device,
                            _p_copy_memory_to_image_info: *const CopyMemoryToImageInfoEXT<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(copy_memory_to_image_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCopyMemoryToImageEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            copy_memory_to_image_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    copy_image_to_memory_ext: unsafe {
                        unsafe extern "system" fn copy_image_to_memory_ext(
                            _device: crate::vk::Device,
                            _p_copy_image_to_memory_info: *const CopyImageToMemoryInfoEXT<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(copy_image_to_memory_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCopyImageToMemoryEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            copy_image_to_memory_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    copy_image_to_image_ext: unsafe {
                        unsafe extern "system" fn copy_image_to_image_ext(
                            _device: crate::vk::Device,
                            _p_copy_image_to_image_info: *const CopyImageToImageInfoEXT<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(copy_image_to_image_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCopyImageToImageEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            copy_image_to_image_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    transition_image_layout_ext: unsafe {
                        unsafe extern "system" fn transition_image_layout_ext(
                            _device: crate::vk::Device,
                            _transition_count: u32,
                            _p_transitions: *const HostImageLayoutTransitionInfoEXT<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(transition_image_layout_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkTransitionImageLayoutEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            transition_image_layout_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_image_subresource_layout2_ext: unsafe {
                        unsafe extern "system" fn get_image_subresource_layout2_ext(
                            _device: crate::vk::Device,
                            _image: Image,
                            _p_subresource: *const ImageSubresource2KHR<'_>,
                            _p_layout: *mut SubresourceLayout2KHR<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_image_subresource_layout2_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetImageSubresourceLayout2EXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_image_subresource_layout2_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_map_memory_placed"]
    pub mod map_memory_placed {
        use super::super::*;
        pub use {
            crate::vk::EXT_MAP_MEMORY_PLACED_NAME as NAME,
            crate::vk::EXT_MAP_MEMORY_PLACED_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_shader_atomic_float2"]
    pub mod shader_atomic_float2 {
        use super::super::*;
        pub use {
            crate::vk::EXT_SHADER_ATOMIC_FLOAT2_NAME as NAME,
            crate::vk::EXT_SHADER_ATOMIC_FLOAT2_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_surface_maintenance1"]
    pub mod surface_maintenance1 {
        use super::super::*;
        pub use {
            crate::vk::EXT_SURFACE_MAINTENANCE1_NAME as NAME,
            crate::vk::EXT_SURFACE_MAINTENANCE1_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_swapchain_maintenance1"]
    pub mod swapchain_maintenance1 {
        use super::super::*;
        pub use {
            crate::vk::EXT_SWAPCHAIN_MAINTENANCE1_NAME as NAME,
            crate::vk::EXT_SWAPCHAIN_MAINTENANCE1_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_swapchain_maintenance1 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_swapchain_maintenance1 device-level function pointers"]
        pub struct DeviceFn {
            pub release_swapchain_images_ext: PFN_vkReleaseSwapchainImagesEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    release_swapchain_images_ext: unsafe {
                        unsafe extern "system" fn release_swapchain_images_ext(
                            _device: crate::vk::Device,
                            _p_release_info: *const ReleaseSwapchainImagesInfoEXT<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(release_swapchain_images_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkReleaseSwapchainImagesEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            release_swapchain_images_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_shader_demote_to_helper_invocation"]
    pub mod shader_demote_to_helper_invocation {
        use super::super::*;
        pub use {
            crate::vk::EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_NAME as NAME,
            crate::vk::EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_texel_buffer_alignment"]
    pub mod texel_buffer_alignment {
        use super::super::*;
        pub use {
            crate::vk::EXT_TEXEL_BUFFER_ALIGNMENT_NAME as NAME,
            crate::vk::EXT_TEXEL_BUFFER_ALIGNMENT_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_depth_bias_control"]
    pub mod depth_bias_control {
        use super::super::*;
        pub use {
            crate::vk::EXT_DEPTH_BIAS_CONTROL_NAME as NAME,
            crate::vk::EXT_DEPTH_BIAS_CONTROL_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_depth_bias_control device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_depth_bias_control device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_set_depth_bias2_ext: PFN_vkCmdSetDepthBias2EXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_set_depth_bias2_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_bias2_ext(
                            _command_buffer: CommandBuffer,
                            _p_depth_bias_info: *const DepthBiasInfoEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_bias2_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthBias2EXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_bias2_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_device_memory_report"]
    pub mod device_memory_report {
        use super::super::*;
        pub use {
            crate::vk::EXT_DEVICE_MEMORY_REPORT_NAME as NAME,
            crate::vk::EXT_DEVICE_MEMORY_REPORT_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_acquire_drm_display"]
    pub mod acquire_drm_display {
        use super::super::*;
        pub use {
            crate::vk::EXT_ACQUIRE_DRM_DISPLAY_NAME as NAME,
            crate::vk::EXT_ACQUIRE_DRM_DISPLAY_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_acquire_drm_display instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_acquire_drm_display instance-level function pointers"]
        pub struct InstanceFn {
            pub acquire_drm_display_ext: PFN_vkAcquireDrmDisplayEXT,
            pub get_drm_display_ext: PFN_vkGetDrmDisplayEXT,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    acquire_drm_display_ext: unsafe {
                        unsafe extern "system" fn acquire_drm_display_ext(
                            _physical_device: PhysicalDevice,
                            _drm_fd: i32,
                            _display: DisplayKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(acquire_drm_display_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkAcquireDrmDisplayEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            acquire_drm_display_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_drm_display_ext: unsafe {
                        unsafe extern "system" fn get_drm_display_ext(
                            _physical_device: PhysicalDevice,
                            _drm_fd: i32,
                            _connector_id: u32,
                            _display: *mut DisplayKHR,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(get_drm_display_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetDrmDisplayEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_drm_display_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_robustness2"]
    pub mod robustness2 {
        use super::super::*;
        pub use {
            crate::vk::EXT_ROBUSTNESS2_NAME as NAME,
            crate::vk::EXT_ROBUSTNESS2_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_custom_border_color"]
    pub mod custom_border_color {
        use super::super::*;
        pub use {
            crate::vk::EXT_CUSTOM_BORDER_COLOR_NAME as NAME,
            crate::vk::EXT_CUSTOM_BORDER_COLOR_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_private_data"]
    pub mod private_data {
        use super::super::*;
        pub use {
            crate::vk::EXT_PRIVATE_DATA_NAME as NAME,
            crate::vk::EXT_PRIVATE_DATA_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_private_data device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_private_data device-level function pointers"]
        pub struct DeviceFn {
            pub create_private_data_slot_ext: PFN_vkCreatePrivateDataSlot,
            pub destroy_private_data_slot_ext: PFN_vkDestroyPrivateDataSlot,
            pub set_private_data_ext: PFN_vkSetPrivateData,
            pub get_private_data_ext: PFN_vkGetPrivateData,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_private_data_slot_ext: unsafe {
                        unsafe extern "system" fn create_private_data_slot_ext(
                            _device: crate::vk::Device,
                            _p_create_info: *const PrivateDataSlotCreateInfo<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_private_data_slot: *mut PrivateDataSlot,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_private_data_slot_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreatePrivateDataSlotEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_private_data_slot_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_private_data_slot_ext: unsafe {
                        unsafe extern "system" fn destroy_private_data_slot_ext(
                            _device: crate::vk::Device,
                            _private_data_slot: PrivateDataSlot,
                            _p_allocator: *const AllocationCallbacks<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(destroy_private_data_slot_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkDestroyPrivateDataSlotEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_private_data_slot_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    set_private_data_ext: unsafe {
                        unsafe extern "system" fn set_private_data_ext(
                            _device: crate::vk::Device,
                            _object_type: ObjectType,
                            _object_handle: u64,
                            _private_data_slot: PrivateDataSlot,
                            _data: u64,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(set_private_data_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkSetPrivateDataEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            set_private_data_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_private_data_ext: unsafe {
                        unsafe extern "system" fn get_private_data_ext(
                            _device: crate::vk::Device,
                            _object_type: ObjectType,
                            _object_handle: u64,
                            _private_data_slot: PrivateDataSlot,
                            _p_data: *mut u64,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(get_private_data_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetPrivateDataEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_private_data_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_pipeline_creation_cache_control"]
    pub mod pipeline_creation_cache_control {
        use super::super::*;
        pub use {
            crate::vk::EXT_PIPELINE_CREATION_CACHE_CONTROL_NAME as NAME,
            crate::vk::EXT_PIPELINE_CREATION_CACHE_CONTROL_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_metal_objects"]
    pub mod metal_objects {
        use super::super::*;
        pub use {
            crate::vk::EXT_METAL_OBJECTS_NAME as NAME,
            crate::vk::EXT_METAL_OBJECTS_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_metal_objects device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_metal_objects device-level function pointers"]
        pub struct DeviceFn {
            pub export_metal_objects_ext: PFN_vkExportMetalObjectsEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    export_metal_objects_ext: unsafe {
                        unsafe extern "system" fn export_metal_objects_ext(
                            _device: crate::vk::Device,
                            _p_metal_objects_info: *mut ExportMetalObjectsInfoEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(export_metal_objects_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkExportMetalObjectsEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            export_metal_objects_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_descriptor_buffer"]
    pub mod descriptor_buffer {
        use super::super::*;
        pub use {
            crate::vk::EXT_DESCRIPTOR_BUFFER_NAME as NAME,
            crate::vk::EXT_DESCRIPTOR_BUFFER_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_descriptor_buffer device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_descriptor_buffer device-level function pointers"]
        pub struct DeviceFn {
            pub get_descriptor_set_layout_size_ext: PFN_vkGetDescriptorSetLayoutSizeEXT,
            pub get_descriptor_set_layout_binding_offset_ext:
                PFN_vkGetDescriptorSetLayoutBindingOffsetEXT,
            pub get_descriptor_ext: PFN_vkGetDescriptorEXT,
            pub cmd_bind_descriptor_buffers_ext: PFN_vkCmdBindDescriptorBuffersEXT,
            pub cmd_set_descriptor_buffer_offsets_ext: PFN_vkCmdSetDescriptorBufferOffsetsEXT,
            pub cmd_bind_descriptor_buffer_embedded_samplers_ext:
                PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT,
            pub get_buffer_opaque_capture_descriptor_data_ext:
                PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT,
            pub get_image_opaque_capture_descriptor_data_ext:
                PFN_vkGetImageOpaqueCaptureDescriptorDataEXT,
            pub get_image_view_opaque_capture_descriptor_data_ext:
                PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT,
            pub get_sampler_opaque_capture_descriptor_data_ext:
                PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT,
            pub get_acceleration_structure_opaque_capture_descriptor_data_ext:
                PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_descriptor_set_layout_size_ext: unsafe {
                        unsafe extern "system" fn get_descriptor_set_layout_size_ext(
                            _device: crate::vk::Device,
                            _layout: DescriptorSetLayout,
                            _p_layout_size_in_bytes: *mut DeviceSize,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_descriptor_set_layout_size_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDescriptorSetLayoutSizeEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_descriptor_set_layout_size_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_descriptor_set_layout_binding_offset_ext: unsafe {
                        unsafe extern "system" fn get_descriptor_set_layout_binding_offset_ext(
                            _device: crate::vk::Device,
                            _layout: DescriptorSetLayout,
                            _binding: u32,
                            _p_offset: *mut DeviceSize,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_descriptor_set_layout_binding_offset_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDescriptorSetLayoutBindingOffsetEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_descriptor_set_layout_binding_offset_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_descriptor_ext: unsafe {
                        unsafe extern "system" fn get_descriptor_ext(
                            _device: crate::vk::Device,
                            _p_descriptor_info: *const DescriptorGetInfoEXT<'_>,
                            _data_size: usize,
                            _p_descriptor: *mut c_void,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(get_descriptor_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetDescriptorEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_descriptor_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_bind_descriptor_buffers_ext: unsafe {
                        unsafe extern "system" fn cmd_bind_descriptor_buffers_ext(
                            _command_buffer: CommandBuffer,
                            _buffer_count: u32,
                            _p_binding_infos: *const DescriptorBufferBindingInfoEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_bind_descriptor_buffers_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdBindDescriptorBuffersEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_bind_descriptor_buffers_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_descriptor_buffer_offsets_ext: unsafe {
                        unsafe extern "system" fn cmd_set_descriptor_buffer_offsets_ext(
                            _command_buffer: CommandBuffer,
                            _pipeline_bind_point: PipelineBindPoint,
                            _layout: PipelineLayout,
                            _first_set: u32,
                            _set_count: u32,
                            _p_buffer_indices: *const u32,
                            _p_offsets: *const DeviceSize,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_descriptor_buffer_offsets_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetDescriptorBufferOffsetsEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_descriptor_buffer_offsets_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_bind_descriptor_buffer_embedded_samplers_ext: unsafe {
                        unsafe extern "system" fn cmd_bind_descriptor_buffer_embedded_samplers_ext(
                            _command_buffer: CommandBuffer,
                            _pipeline_bind_point: PipelineBindPoint,
                            _layout: PipelineLayout,
                            _set: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_bind_descriptor_buffer_embedded_samplers_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdBindDescriptorBufferEmbeddedSamplersEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_bind_descriptor_buffer_embedded_samplers_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_buffer_opaque_capture_descriptor_data_ext: unsafe {
                        unsafe extern "system" fn get_buffer_opaque_capture_descriptor_data_ext(
                            _device: crate::vk::Device,
                            _p_info: *const BufferCaptureDescriptorDataInfoEXT<'_>,
                            _p_data: *mut c_void,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_buffer_opaque_capture_descriptor_data_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetBufferOpaqueCaptureDescriptorDataEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_buffer_opaque_capture_descriptor_data_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_image_opaque_capture_descriptor_data_ext: unsafe {
                        unsafe extern "system" fn get_image_opaque_capture_descriptor_data_ext(
                            _device: crate::vk::Device,
                            _p_info: *const ImageCaptureDescriptorDataInfoEXT<'_>,
                            _p_data: *mut c_void,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_image_opaque_capture_descriptor_data_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetImageOpaqueCaptureDescriptorDataEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_image_opaque_capture_descriptor_data_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_image_view_opaque_capture_descriptor_data_ext: unsafe {
                        unsafe extern "system" fn get_image_view_opaque_capture_descriptor_data_ext(
                            _device: crate::vk::Device,
                            _p_info: *const ImageViewCaptureDescriptorDataInfoEXT<'_>,
                            _p_data: *mut c_void,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_image_view_opaque_capture_descriptor_data_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetImageViewOpaqueCaptureDescriptorDataEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_image_view_opaque_capture_descriptor_data_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_sampler_opaque_capture_descriptor_data_ext: unsafe {
                        unsafe extern "system" fn get_sampler_opaque_capture_descriptor_data_ext(
                            _device: crate::vk::Device,
                            _p_info: *const SamplerCaptureDescriptorDataInfoEXT<'_>,
                            _p_data: *mut c_void,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_sampler_opaque_capture_descriptor_data_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetSamplerOpaqueCaptureDescriptorDataEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_sampler_opaque_capture_descriptor_data_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_acceleration_structure_opaque_capture_descriptor_data_ext: unsafe {
                        unsafe extern "system" fn get_acceleration_structure_opaque_capture_descriptor_data_ext(
                            _device: crate::vk::Device,
                            _p_info: *const AccelerationStructureCaptureDescriptorDataInfoEXT<'_>,
                            _p_data: *mut c_void,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(
                                    get_acceleration_structure_opaque_capture_descriptor_data_ext
                                )
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_acceleration_structure_opaque_capture_descriptor_data_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_graphics_pipeline_library"]
    pub mod graphics_pipeline_library {
        use super::super::*;
        pub use {
            crate::vk::EXT_GRAPHICS_PIPELINE_LIBRARY_NAME as NAME,
            crate::vk::EXT_GRAPHICS_PIPELINE_LIBRARY_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_mesh_shader"]
    pub mod mesh_shader {
        use super::super::*;
        pub use {
            crate::vk::EXT_MESH_SHADER_NAME as NAME,
            crate::vk::EXT_MESH_SHADER_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_mesh_shader device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_mesh_shader device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_draw_mesh_tasks_ext: PFN_vkCmdDrawMeshTasksEXT,
            pub cmd_draw_mesh_tasks_indirect_ext: PFN_vkCmdDrawMeshTasksIndirectEXT,
            pub cmd_draw_mesh_tasks_indirect_count_ext: PFN_vkCmdDrawMeshTasksIndirectCountEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_draw_mesh_tasks_ext: unsafe {
                        unsafe extern "system" fn cmd_draw_mesh_tasks_ext(
                            _command_buffer: CommandBuffer,
                            _group_count_x: u32,
                            _group_count_y: u32,
                            _group_count_z: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_draw_mesh_tasks_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawMeshTasksEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_draw_mesh_tasks_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_draw_mesh_tasks_indirect_ext: unsafe {
                        unsafe extern "system" fn cmd_draw_mesh_tasks_indirect_ext(
                            _command_buffer: CommandBuffer,
                            _buffer: Buffer,
                            _offset: DeviceSize,
                            _draw_count: u32,
                            _stride: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_draw_mesh_tasks_indirect_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawMeshTasksIndirectEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_draw_mesh_tasks_indirect_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_draw_mesh_tasks_indirect_count_ext: unsafe {
                        unsafe extern "system" fn cmd_draw_mesh_tasks_indirect_count_ext(
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
                                stringify!(cmd_draw_mesh_tasks_indirect_count_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdDrawMeshTasksIndirectCountEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_draw_mesh_tasks_indirect_count_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_ycbcr_2plane_444_formats"]
    pub mod ycbcr_2plane_444_formats {
        use super::super::*;
        pub use {
            crate::vk::EXT_YCBCR_2PLANE_444_FORMATS_NAME as NAME,
            crate::vk::EXT_YCBCR_2PLANE_444_FORMATS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_fragment_density_map2"]
    pub mod fragment_density_map2 {
        use super::super::*;
        pub use {
            crate::vk::EXT_FRAGMENT_DENSITY_MAP2_NAME as NAME,
            crate::vk::EXT_FRAGMENT_DENSITY_MAP2_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_image_robustness"]
    pub mod image_robustness {
        use super::super::*;
        pub use {
            crate::vk::EXT_IMAGE_ROBUSTNESS_NAME as NAME,
            crate::vk::EXT_IMAGE_ROBUSTNESS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_image_compression_control"]
    pub mod image_compression_control {
        use super::super::*;
        pub use {
            crate::vk::EXT_IMAGE_COMPRESSION_CONTROL_NAME as NAME,
            crate::vk::EXT_IMAGE_COMPRESSION_CONTROL_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_image_compression_control device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_image_compression_control device-level function pointers"]
        pub struct DeviceFn {
            pub get_image_subresource_layout2_ext: PFN_vkGetImageSubresourceLayout2KHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_image_subresource_layout2_ext: unsafe {
                        unsafe extern "system" fn get_image_subresource_layout2_ext(
                            _device: crate::vk::Device,
                            _image: Image,
                            _p_subresource: *const ImageSubresource2KHR<'_>,
                            _p_layout: *mut SubresourceLayout2KHR<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_image_subresource_layout2_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetImageSubresourceLayout2EXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_image_subresource_layout2_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_attachment_feedback_loop_layout"]
    pub mod attachment_feedback_loop_layout {
        use super::super::*;
        pub use {
            crate::vk::EXT_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_NAME as NAME,
            crate::vk::EXT_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_4444_formats"]
    pub mod _4444_formats {
        use super::super::*;
        pub use {
            crate::vk::EXT_4444_FORMATS_NAME as NAME,
            crate::vk::EXT_4444_FORMATS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_device_fault"]
    pub mod device_fault {
        use super::super::*;
        pub use {
            crate::vk::EXT_DEVICE_FAULT_NAME as NAME,
            crate::vk::EXT_DEVICE_FAULT_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_device_fault device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_device_fault device-level function pointers"]
        pub struct DeviceFn {
            pub get_device_fault_info_ext: PFN_vkGetDeviceFaultInfoEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_device_fault_info_ext: unsafe {
                        unsafe extern "system" fn get_device_fault_info_ext(
                            _device: crate::vk::Device,
                            _p_fault_counts: *mut DeviceFaultCountsEXT<'_>,
                            _p_fault_info: *mut DeviceFaultInfoEXT<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_device_fault_info_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceFaultInfoEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_device_fault_info_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_rgba10x6_formats"]
    pub mod rgba10x6_formats {
        use super::super::*;
        pub use {
            crate::vk::EXT_RGBA10X6_FORMATS_NAME as NAME,
            crate::vk::EXT_RGBA10X6_FORMATS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_directfb_surface"]
    pub mod directfb_surface {
        use super::super::*;
        pub use {
            crate::vk::EXT_DIRECTFB_SURFACE_NAME as NAME,
            crate::vk::EXT_DIRECTFB_SURFACE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_directfb_surface instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_directfb_surface instance-level function pointers"]
        pub struct InstanceFn {
            pub create_direct_fb_surface_ext: PFN_vkCreateDirectFBSurfaceEXT,
            pub get_physical_device_direct_fb_presentation_support_ext:
                PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_direct_fb_surface_ext: unsafe {
                        unsafe extern "system" fn create_direct_fb_surface_ext(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const DirectFBSurfaceCreateInfoEXT<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_surface: *mut SurfaceKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_direct_fb_surface_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateDirectFBSurfaceEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_direct_fb_surface_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_direct_fb_presentation_support_ext: unsafe {
                        unsafe extern "system" fn get_physical_device_direct_fb_presentation_support_ext(
                            _physical_device: PhysicalDevice,
                            _queue_family_index: u32,
                            _dfb: *mut IDirectFB,
                        ) -> Bool32 {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_direct_fb_presentation_support_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceDirectFBPresentationSupportEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_direct_fb_presentation_support_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_vertex_input_dynamic_state"]
    pub mod vertex_input_dynamic_state {
        use super::super::*;
        pub use {
            crate::vk::EXT_VERTEX_INPUT_DYNAMIC_STATE_NAME as NAME,
            crate::vk::EXT_VERTEX_INPUT_DYNAMIC_STATE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_vertex_input_dynamic_state device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_vertex_input_dynamic_state device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_set_vertex_input_ext: PFN_vkCmdSetVertexInputEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_set_vertex_input_ext: unsafe {
                        unsafe extern "system" fn cmd_set_vertex_input_ext(
                            _command_buffer: CommandBuffer,
                            _vertex_binding_description_count: u32,
                            _p_vertex_binding_descriptions : * const VertexInputBindingDescription2EXT < '_ >,
                            _vertex_attribute_description_count: u32,
                            _p_vertex_attribute_descriptions : * const VertexInputAttributeDescription2EXT < '_ >,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_vertex_input_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetVertexInputEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_vertex_input_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_physical_device_drm"]
    pub mod physical_device_drm {
        use super::super::*;
        pub use {
            crate::vk::EXT_PHYSICAL_DEVICE_DRM_NAME as NAME,
            crate::vk::EXT_PHYSICAL_DEVICE_DRM_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_device_address_binding_report"]
    pub mod device_address_binding_report {
        use super::super::*;
        pub use {
            crate::vk::EXT_DEVICE_ADDRESS_BINDING_REPORT_NAME as NAME,
            crate::vk::EXT_DEVICE_ADDRESS_BINDING_REPORT_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_depth_clip_control"]
    pub mod depth_clip_control {
        use super::super::*;
        pub use {
            crate::vk::EXT_DEPTH_CLIP_CONTROL_NAME as NAME,
            crate::vk::EXT_DEPTH_CLIP_CONTROL_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_primitive_topology_list_restart"]
    pub mod primitive_topology_list_restart {
        use super::super::*;
        pub use {
            crate::vk::EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_NAME as NAME,
            crate::vk::EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_pipeline_properties"]
    pub mod pipeline_properties {
        use super::super::*;
        pub use {
            crate::vk::EXT_PIPELINE_PROPERTIES_NAME as NAME,
            crate::vk::EXT_PIPELINE_PROPERTIES_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_pipeline_properties device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[allow(non_camel_case_types)]
        #[doc = "Implemented for all types that can be passed as argument to `pipeline_properties` in [`PFN_vkGetPipelinePropertiesEXT`]"]
        pub unsafe trait GetPipelinePropertiesEXTParamPipelineProperties {}
        unsafe impl GetPipelinePropertiesEXTParamPipelineProperties
            for PipelinePropertiesIdentifierEXT<'_>
        {
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_pipeline_properties device-level function pointers"]
        pub struct DeviceFn {
            pub get_pipeline_properties_ext: PFN_vkGetPipelinePropertiesEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_pipeline_properties_ext: unsafe {
                        unsafe extern "system" fn get_pipeline_properties_ext(
                            _device: crate::vk::Device,
                            _p_pipeline_info: *const PipelineInfoEXT<'_>,
                            _p_pipeline_properties: *mut BaseOutStructure<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_pipeline_properties_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetPipelinePropertiesEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_pipeline_properties_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_frame_boundary"]
    pub mod frame_boundary {
        use super::super::*;
        pub use {
            crate::vk::EXT_FRAME_BOUNDARY_NAME as NAME,
            crate::vk::EXT_FRAME_BOUNDARY_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_multisampled_render_to_single_sampled"]
    pub mod multisampled_render_to_single_sampled {
        use super::super::*;
        pub use {
            crate::vk::EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_NAME as NAME,
            crate::vk::EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_extended_dynamic_state2"]
    pub mod extended_dynamic_state2 {
        use super::super::*;
        pub use {
            crate::vk::EXT_EXTENDED_DYNAMIC_STATE2_NAME as NAME,
            crate::vk::EXT_EXTENDED_DYNAMIC_STATE2_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_extended_dynamic_state2 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_extended_dynamic_state2 device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_set_patch_control_points_ext: PFN_vkCmdSetPatchControlPointsEXT,
            pub cmd_set_rasterizer_discard_enable_ext: PFN_vkCmdSetRasterizerDiscardEnable,
            pub cmd_set_depth_bias_enable_ext: PFN_vkCmdSetDepthBiasEnable,
            pub cmd_set_logic_op_ext: PFN_vkCmdSetLogicOpEXT,
            pub cmd_set_primitive_restart_enable_ext: PFN_vkCmdSetPrimitiveRestartEnable,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_set_patch_control_points_ext: unsafe {
                        unsafe extern "system" fn cmd_set_patch_control_points_ext(
                            _command_buffer: CommandBuffer,
                            _patch_control_points: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_patch_control_points_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetPatchControlPointsEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_patch_control_points_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_rasterizer_discard_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_rasterizer_discard_enable_ext(
                            _command_buffer: CommandBuffer,
                            _rasterizer_discard_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_rasterizer_discard_enable_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetRasterizerDiscardEnableEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_rasterizer_discard_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_bias_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_bias_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_bias_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_bias_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthBiasEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_bias_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_logic_op_ext: unsafe {
                        unsafe extern "system" fn cmd_set_logic_op_ext(
                            _command_buffer: CommandBuffer,
                            _logic_op: LogicOp,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_set_logic_op_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLogicOpEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_logic_op_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_primitive_restart_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_primitive_restart_enable_ext(
                            _command_buffer: CommandBuffer,
                            _primitive_restart_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_primitive_restart_enable_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetPrimitiveRestartEnableEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_primitive_restart_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_color_write_enable"]
    pub mod color_write_enable {
        use super::super::*;
        pub use {
            crate::vk::EXT_COLOR_WRITE_ENABLE_NAME as NAME,
            crate::vk::EXT_COLOR_WRITE_ENABLE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_color_write_enable device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_color_write_enable device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_set_color_write_enable_ext: PFN_vkCmdSetColorWriteEnableEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_set_color_write_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_color_write_enable_ext(
                            _command_buffer: CommandBuffer,
                            _attachment_count: u32,
                            _p_color_write_enables: *const Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_color_write_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetColorWriteEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_color_write_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_primitives_generated_query"]
    pub mod primitives_generated_query {
        use super::super::*;
        pub use {
            crate::vk::EXT_PRIMITIVES_GENERATED_QUERY_NAME as NAME,
            crate::vk::EXT_PRIMITIVES_GENERATED_QUERY_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_global_priority_query"]
    pub mod global_priority_query {
        use super::super::*;
        pub use {
            crate::vk::EXT_GLOBAL_PRIORITY_QUERY_NAME as NAME,
            crate::vk::EXT_GLOBAL_PRIORITY_QUERY_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_image_view_min_lod"]
    pub mod image_view_min_lod {
        use super::super::*;
        pub use {
            crate::vk::EXT_IMAGE_VIEW_MIN_LOD_NAME as NAME,
            crate::vk::EXT_IMAGE_VIEW_MIN_LOD_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_multi_draw"]
    pub mod multi_draw {
        use super::super::*;
        pub use {
            crate::vk::EXT_MULTI_DRAW_NAME as NAME,
            crate::vk::EXT_MULTI_DRAW_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_multi_draw device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_multi_draw device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_draw_multi_ext: PFN_vkCmdDrawMultiEXT,
            pub cmd_draw_multi_indexed_ext: PFN_vkCmdDrawMultiIndexedEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_draw_multi_ext: unsafe {
                        unsafe extern "system" fn cmd_draw_multi_ext(
                            _command_buffer: CommandBuffer,
                            _draw_count: u32,
                            _p_vertex_info: *const MultiDrawInfoEXT,
                            _instance_count: u32,
                            _first_instance: u32,
                            _stride: u32,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_draw_multi_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawMultiEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_draw_multi_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_draw_multi_indexed_ext: unsafe {
                        unsafe extern "system" fn cmd_draw_multi_indexed_ext(
                            _command_buffer: CommandBuffer,
                            _draw_count: u32,
                            _p_index_info: *const MultiDrawIndexedInfoEXT,
                            _instance_count: u32,
                            _first_instance: u32,
                            _stride: u32,
                            _p_vertex_offset: *const i32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_draw_multi_indexed_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawMultiIndexedEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_draw_multi_indexed_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_image_2d_view_of_3d"]
    pub mod image_2d_view_of_3d {
        use super::super::*;
        pub use {
            crate::vk::EXT_IMAGE_2D_VIEW_OF_3D_NAME as NAME,
            crate::vk::EXT_IMAGE_2D_VIEW_OF_3D_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_shader_tile_image"]
    pub mod shader_tile_image {
        use super::super::*;
        pub use {
            crate::vk::EXT_SHADER_TILE_IMAGE_NAME as NAME,
            crate::vk::EXT_SHADER_TILE_IMAGE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_opacity_micromap"]
    pub mod opacity_micromap {
        use super::super::*;
        pub use {
            crate::vk::EXT_OPACITY_MICROMAP_NAME as NAME,
            crate::vk::EXT_OPACITY_MICROMAP_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_opacity_micromap device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_opacity_micromap device-level function pointers"]
        pub struct DeviceFn {
            pub create_micromap_ext: PFN_vkCreateMicromapEXT,
            pub destroy_micromap_ext: PFN_vkDestroyMicromapEXT,
            pub cmd_build_micromaps_ext: PFN_vkCmdBuildMicromapsEXT,
            pub build_micromaps_ext: PFN_vkBuildMicromapsEXT,
            pub copy_micromap_ext: PFN_vkCopyMicromapEXT,
            pub copy_micromap_to_memory_ext: PFN_vkCopyMicromapToMemoryEXT,
            pub copy_memory_to_micromap_ext: PFN_vkCopyMemoryToMicromapEXT,
            pub write_micromaps_properties_ext: PFN_vkWriteMicromapsPropertiesEXT,
            pub cmd_copy_micromap_ext: PFN_vkCmdCopyMicromapEXT,
            pub cmd_copy_micromap_to_memory_ext: PFN_vkCmdCopyMicromapToMemoryEXT,
            pub cmd_copy_memory_to_micromap_ext: PFN_vkCmdCopyMemoryToMicromapEXT,
            pub cmd_write_micromaps_properties_ext: PFN_vkCmdWriteMicromapsPropertiesEXT,
            pub get_device_micromap_compatibility_ext: PFN_vkGetDeviceMicromapCompatibilityEXT,
            pub get_micromap_build_sizes_ext: PFN_vkGetMicromapBuildSizesEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_micromap_ext: unsafe {
                        unsafe extern "system" fn create_micromap_ext(
                            _device: crate::vk::Device,
                            _p_create_info: *const MicromapCreateInfoEXT<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_micromap: *mut MicromapEXT,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(create_micromap_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateMicromapEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_micromap_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_micromap_ext: unsafe {
                        unsafe extern "system" fn destroy_micromap_ext(
                            _device: crate::vk::Device,
                            _micromap: MicromapEXT,
                            _p_allocator: *const AllocationCallbacks<'_>,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(destroy_micromap_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyMicromapEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_micromap_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_build_micromaps_ext: unsafe {
                        unsafe extern "system" fn cmd_build_micromaps_ext(
                            _command_buffer: CommandBuffer,
                            _info_count: u32,
                            _p_infos: *const MicromapBuildInfoEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_build_micromaps_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdBuildMicromapsEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_build_micromaps_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    build_micromaps_ext: unsafe {
                        unsafe extern "system" fn build_micromaps_ext(
                            _device: crate::vk::Device,
                            _deferred_operation: DeferredOperationKHR,
                            _info_count: u32,
                            _p_infos: *const MicromapBuildInfoEXT<'_>,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(build_micromaps_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkBuildMicromapsEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            build_micromaps_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    copy_micromap_ext: unsafe {
                        unsafe extern "system" fn copy_micromap_ext(
                            _device: crate::vk::Device,
                            _deferred_operation: DeferredOperationKHR,
                            _p_info: *const CopyMicromapInfoEXT<'_>,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(copy_micromap_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCopyMicromapEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            copy_micromap_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    copy_micromap_to_memory_ext: unsafe {
                        unsafe extern "system" fn copy_micromap_to_memory_ext(
                            _device: crate::vk::Device,
                            _deferred_operation: DeferredOperationKHR,
                            _p_info: *const CopyMicromapToMemoryInfoEXT<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(copy_micromap_to_memory_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCopyMicromapToMemoryEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            copy_micromap_to_memory_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    copy_memory_to_micromap_ext: unsafe {
                        unsafe extern "system" fn copy_memory_to_micromap_ext(
                            _device: crate::vk::Device,
                            _deferred_operation: DeferredOperationKHR,
                            _p_info: *const CopyMemoryToMicromapInfoEXT<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(copy_memory_to_micromap_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCopyMemoryToMicromapEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            copy_memory_to_micromap_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    write_micromaps_properties_ext: unsafe {
                        unsafe extern "system" fn write_micromaps_properties_ext(
                            _device: crate::vk::Device,
                            _micromap_count: u32,
                            _p_micromaps: *const MicromapEXT,
                            _query_type: QueryType,
                            _data_size: usize,
                            _p_data: *mut c_void,
                            _stride: usize,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(write_micromaps_properties_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkWriteMicromapsPropertiesEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            write_micromaps_properties_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_copy_micromap_ext: unsafe {
                        unsafe extern "system" fn cmd_copy_micromap_ext(
                            _command_buffer: CommandBuffer,
                            _p_info: *const CopyMicromapInfoEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_copy_micromap_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyMicromapEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_copy_micromap_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_copy_micromap_to_memory_ext: unsafe {
                        unsafe extern "system" fn cmd_copy_micromap_to_memory_ext(
                            _command_buffer: CommandBuffer,
                            _p_info: *const CopyMicromapToMemoryInfoEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_copy_micromap_to_memory_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyMicromapToMemoryEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_copy_micromap_to_memory_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_copy_memory_to_micromap_ext: unsafe {
                        unsafe extern "system" fn cmd_copy_memory_to_micromap_ext(
                            _command_buffer: CommandBuffer,
                            _p_info: *const CopyMemoryToMicromapInfoEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_copy_memory_to_micromap_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyMemoryToMicromapEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_copy_memory_to_micromap_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_write_micromaps_properties_ext: unsafe {
                        unsafe extern "system" fn cmd_write_micromaps_properties_ext(
                            _command_buffer: CommandBuffer,
                            _micromap_count: u32,
                            _p_micromaps: *const MicromapEXT,
                            _query_type: QueryType,
                            _query_pool: QueryPool,
                            _first_query: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_write_micromaps_properties_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdWriteMicromapsPropertiesEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_write_micromaps_properties_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_device_micromap_compatibility_ext: unsafe {
                        unsafe extern "system" fn get_device_micromap_compatibility_ext(
                            _device: crate::vk::Device,
                            _p_version_info: *const MicromapVersionInfoEXT<'_>,
                            _p_compatibility: *mut AccelerationStructureCompatibilityKHR,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_device_micromap_compatibility_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeviceMicromapCompatibilityEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_device_micromap_compatibility_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_micromap_build_sizes_ext: unsafe {
                        unsafe extern "system" fn get_micromap_build_sizes_ext(
                            _device: crate::vk::Device,
                            _build_type: AccelerationStructureBuildTypeKHR,
                            _p_build_info: *const MicromapBuildInfoEXT<'_>,
                            _p_size_info: *mut MicromapBuildSizesInfoEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_micromap_build_sizes_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetMicromapBuildSizesEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_micromap_build_sizes_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_load_store_op_none"]
    pub mod load_store_op_none {
        use super::super::*;
        pub use {
            crate::vk::EXT_LOAD_STORE_OP_NONE_NAME as NAME,
            crate::vk::EXT_LOAD_STORE_OP_NONE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_border_color_swizzle"]
    pub mod border_color_swizzle {
        use super::super::*;
        pub use {
            crate::vk::EXT_BORDER_COLOR_SWIZZLE_NAME as NAME,
            crate::vk::EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_pageable_device_local_memory"]
    pub mod pageable_device_local_memory {
        use super::super::*;
        pub use {
            crate::vk::EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_NAME as NAME,
            crate::vk::EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_pageable_device_local_memory device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_pageable_device_local_memory device-level function pointers"]
        pub struct DeviceFn {
            pub set_device_memory_priority_ext: PFN_vkSetDeviceMemoryPriorityEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    set_device_memory_priority_ext: unsafe {
                        unsafe extern "system" fn set_device_memory_priority_ext(
                            _device: crate::vk::Device,
                            _memory: DeviceMemory,
                            _priority: f32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(set_device_memory_priority_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkSetDeviceMemoryPriorityEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            set_device_memory_priority_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_image_sliced_view_of_3d"]
    pub mod image_sliced_view_of_3d {
        use super::super::*;
        pub use {
            crate::vk::EXT_IMAGE_SLICED_VIEW_OF_3D_NAME as NAME,
            crate::vk::EXT_IMAGE_SLICED_VIEW_OF_3D_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_depth_clamp_zero_one"]
    pub mod depth_clamp_zero_one {
        use super::super::*;
        pub use {
            crate::vk::EXT_DEPTH_CLAMP_ZERO_ONE_NAME as NAME,
            crate::vk::EXT_DEPTH_CLAMP_ZERO_ONE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_non_seamless_cube_map"]
    pub mod non_seamless_cube_map {
        use super::super::*;
        pub use {
            crate::vk::EXT_NON_SEAMLESS_CUBE_MAP_NAME as NAME,
            crate::vk::EXT_NON_SEAMLESS_CUBE_MAP_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_image_compression_control_swapchain"]
    pub mod image_compression_control_swapchain {
        use super::super::*;
        pub use {
            crate::vk::EXT_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_NAME as NAME,
            crate::vk::EXT_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_nested_command_buffer"]
    pub mod nested_command_buffer {
        use super::super::*;
        pub use {
            crate::vk::EXT_NESTED_COMMAND_BUFFER_NAME as NAME,
            crate::vk::EXT_NESTED_COMMAND_BUFFER_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_external_memory_acquire_unmodified"]
    pub mod external_memory_acquire_unmodified {
        use super::super::*;
        pub use {
            crate::vk::EXT_EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_NAME as NAME,
            crate::vk::EXT_EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_extended_dynamic_state3"]
    pub mod extended_dynamic_state3 {
        use super::super::*;
        pub use {
            crate::vk::EXT_EXTENDED_DYNAMIC_STATE3_NAME as NAME,
            crate::vk::EXT_EXTENDED_DYNAMIC_STATE3_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_extended_dynamic_state3 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_extended_dynamic_state3 device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_set_depth_clamp_enable_ext: PFN_vkCmdSetDepthClampEnableEXT,
            pub cmd_set_polygon_mode_ext: PFN_vkCmdSetPolygonModeEXT,
            pub cmd_set_rasterization_samples_ext: PFN_vkCmdSetRasterizationSamplesEXT,
            pub cmd_set_sample_mask_ext: PFN_vkCmdSetSampleMaskEXT,
            pub cmd_set_alpha_to_coverage_enable_ext: PFN_vkCmdSetAlphaToCoverageEnableEXT,
            pub cmd_set_alpha_to_one_enable_ext: PFN_vkCmdSetAlphaToOneEnableEXT,
            pub cmd_set_logic_op_enable_ext: PFN_vkCmdSetLogicOpEnableEXT,
            pub cmd_set_color_blend_enable_ext: PFN_vkCmdSetColorBlendEnableEXT,
            pub cmd_set_color_blend_equation_ext: PFN_vkCmdSetColorBlendEquationEXT,
            pub cmd_set_color_write_mask_ext: PFN_vkCmdSetColorWriteMaskEXT,
            pub cmd_set_tessellation_domain_origin_ext: PFN_vkCmdSetTessellationDomainOriginEXT,
            pub cmd_set_rasterization_stream_ext: PFN_vkCmdSetRasterizationStreamEXT,
            pub cmd_set_conservative_rasterization_mode_ext:
                PFN_vkCmdSetConservativeRasterizationModeEXT,
            pub cmd_set_extra_primitive_overestimation_size_ext:
                PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT,
            pub cmd_set_depth_clip_enable_ext: PFN_vkCmdSetDepthClipEnableEXT,
            pub cmd_set_sample_locations_enable_ext: PFN_vkCmdSetSampleLocationsEnableEXT,
            pub cmd_set_color_blend_advanced_ext: PFN_vkCmdSetColorBlendAdvancedEXT,
            pub cmd_set_provoking_vertex_mode_ext: PFN_vkCmdSetProvokingVertexModeEXT,
            pub cmd_set_line_rasterization_mode_ext: PFN_vkCmdSetLineRasterizationModeEXT,
            pub cmd_set_line_stipple_enable_ext: PFN_vkCmdSetLineStippleEnableEXT,
            pub cmd_set_depth_clip_negative_one_to_one_ext:
                PFN_vkCmdSetDepthClipNegativeOneToOneEXT,
            pub cmd_set_viewport_w_scaling_enable_nv: PFN_vkCmdSetViewportWScalingEnableNV,
            pub cmd_set_viewport_swizzle_nv: PFN_vkCmdSetViewportSwizzleNV,
            pub cmd_set_coverage_to_color_enable_nv: PFN_vkCmdSetCoverageToColorEnableNV,
            pub cmd_set_coverage_to_color_location_nv: PFN_vkCmdSetCoverageToColorLocationNV,
            pub cmd_set_coverage_modulation_mode_nv: PFN_vkCmdSetCoverageModulationModeNV,
            pub cmd_set_coverage_modulation_table_enable_nv:
                PFN_vkCmdSetCoverageModulationTableEnableNV,
            pub cmd_set_coverage_modulation_table_nv: PFN_vkCmdSetCoverageModulationTableNV,
            pub cmd_set_shading_rate_image_enable_nv: PFN_vkCmdSetShadingRateImageEnableNV,
            pub cmd_set_representative_fragment_test_enable_nv:
                PFN_vkCmdSetRepresentativeFragmentTestEnableNV,
            pub cmd_set_coverage_reduction_mode_nv: PFN_vkCmdSetCoverageReductionModeNV,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_set_depth_clamp_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_clamp_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_clamp_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_clamp_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthClampEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_clamp_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_polygon_mode_ext: unsafe {
                        unsafe extern "system" fn cmd_set_polygon_mode_ext(
                            _command_buffer: CommandBuffer,
                            _polygon_mode: PolygonMode,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_polygon_mode_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetPolygonModeEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_polygon_mode_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_rasterization_samples_ext: unsafe {
                        unsafe extern "system" fn cmd_set_rasterization_samples_ext(
                            _command_buffer: CommandBuffer,
                            _rasterization_samples: SampleCountFlags,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_rasterization_samples_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetRasterizationSamplesEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_rasterization_samples_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_sample_mask_ext: unsafe {
                        unsafe extern "system" fn cmd_set_sample_mask_ext(
                            _command_buffer: CommandBuffer,
                            _samples: SampleCountFlags,
                            _p_sample_mask: *const SampleMask,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_sample_mask_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetSampleMaskEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_sample_mask_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_alpha_to_coverage_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_alpha_to_coverage_enable_ext(
                            _command_buffer: CommandBuffer,
                            _alpha_to_coverage_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_alpha_to_coverage_enable_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetAlphaToCoverageEnableEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_alpha_to_coverage_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_alpha_to_one_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_alpha_to_one_enable_ext(
                            _command_buffer: CommandBuffer,
                            _alpha_to_one_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_alpha_to_one_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetAlphaToOneEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_alpha_to_one_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_logic_op_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_logic_op_enable_ext(
                            _command_buffer: CommandBuffer,
                            _logic_op_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_logic_op_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLogicOpEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_logic_op_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_color_blend_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_color_blend_enable_ext(
                            _command_buffer: CommandBuffer,
                            _first_attachment: u32,
                            _attachment_count: u32,
                            _p_color_blend_enables: *const Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_color_blend_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetColorBlendEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_color_blend_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_color_blend_equation_ext: unsafe {
                        unsafe extern "system" fn cmd_set_color_blend_equation_ext(
                            _command_buffer: CommandBuffer,
                            _first_attachment: u32,
                            _attachment_count: u32,
                            _p_color_blend_equations: *const ColorBlendEquationEXT,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_color_blend_equation_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetColorBlendEquationEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_color_blend_equation_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_color_write_mask_ext: unsafe {
                        unsafe extern "system" fn cmd_set_color_write_mask_ext(
                            _command_buffer: CommandBuffer,
                            _first_attachment: u32,
                            _attachment_count: u32,
                            _p_color_write_masks: *const ColorComponentFlags,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_color_write_mask_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetColorWriteMaskEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_color_write_mask_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_tessellation_domain_origin_ext: unsafe {
                        unsafe extern "system" fn cmd_set_tessellation_domain_origin_ext(
                            _command_buffer: CommandBuffer,
                            _domain_origin: TessellationDomainOrigin,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_tessellation_domain_origin_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetTessellationDomainOriginEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_tessellation_domain_origin_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_rasterization_stream_ext: unsafe {
                        unsafe extern "system" fn cmd_set_rasterization_stream_ext(
                            _command_buffer: CommandBuffer,
                            _rasterization_stream: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_rasterization_stream_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetRasterizationStreamEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_rasterization_stream_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_conservative_rasterization_mode_ext: unsafe {
                        unsafe extern "system" fn cmd_set_conservative_rasterization_mode_ext(
                            _command_buffer: CommandBuffer,
                            _conservative_rasterization_mode: ConservativeRasterizationModeEXT,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_conservative_rasterization_mode_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetConservativeRasterizationModeEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_conservative_rasterization_mode_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_extra_primitive_overestimation_size_ext: unsafe {
                        unsafe extern "system" fn cmd_set_extra_primitive_overestimation_size_ext(
                            _command_buffer: CommandBuffer,
                            _extra_primitive_overestimation_size: f32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_extra_primitive_overestimation_size_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetExtraPrimitiveOverestimationSizeEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_extra_primitive_overestimation_size_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_clip_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_clip_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_clip_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_clip_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthClipEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_clip_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_sample_locations_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_sample_locations_enable_ext(
                            _command_buffer: CommandBuffer,
                            _sample_locations_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_sample_locations_enable_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetSampleLocationsEnableEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_sample_locations_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_color_blend_advanced_ext: unsafe {
                        unsafe extern "system" fn cmd_set_color_blend_advanced_ext(
                            _command_buffer: CommandBuffer,
                            _first_attachment: u32,
                            _attachment_count: u32,
                            _p_color_blend_advanced: *const ColorBlendAdvancedEXT,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_color_blend_advanced_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetColorBlendAdvancedEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_color_blend_advanced_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_provoking_vertex_mode_ext: unsafe {
                        unsafe extern "system" fn cmd_set_provoking_vertex_mode_ext(
                            _command_buffer: CommandBuffer,
                            _provoking_vertex_mode: ProvokingVertexModeEXT,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_provoking_vertex_mode_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetProvokingVertexModeEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_provoking_vertex_mode_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_line_rasterization_mode_ext: unsafe {
                        unsafe extern "system" fn cmd_set_line_rasterization_mode_ext(
                            _command_buffer: CommandBuffer,
                            _line_rasterization_mode: LineRasterizationModeEXT,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_line_rasterization_mode_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetLineRasterizationModeEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_line_rasterization_mode_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_line_stipple_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_line_stipple_enable_ext(
                            _command_buffer: CommandBuffer,
                            _stippled_line_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_line_stipple_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLineStippleEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_line_stipple_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_clip_negative_one_to_one_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_clip_negative_one_to_one_ext(
                            _command_buffer: CommandBuffer,
                            _negative_one_to_one: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_clip_negative_one_to_one_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetDepthClipNegativeOneToOneEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_clip_negative_one_to_one_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_viewport_w_scaling_enable_nv: unsafe {
                        unsafe extern "system" fn cmd_set_viewport_w_scaling_enable_nv(
                            _command_buffer: CommandBuffer,
                            _viewport_w_scaling_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_viewport_w_scaling_enable_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetViewportWScalingEnableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_viewport_w_scaling_enable_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_viewport_swizzle_nv: unsafe {
                        unsafe extern "system" fn cmd_set_viewport_swizzle_nv(
                            _command_buffer: CommandBuffer,
                            _first_viewport: u32,
                            _viewport_count: u32,
                            _p_viewport_swizzles: *const ViewportSwizzleNV,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_viewport_swizzle_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetViewportSwizzleNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_viewport_swizzle_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_to_color_enable_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_to_color_enable_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_to_color_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_to_color_enable_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageToColorEnableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_to_color_enable_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_to_color_location_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_to_color_location_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_to_color_location: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_to_color_location_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageToColorLocationNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_to_color_location_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_modulation_mode_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_modulation_mode_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_modulation_mode: CoverageModulationModeNV,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_modulation_mode_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageModulationModeNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_modulation_mode_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_modulation_table_enable_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_modulation_table_enable_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_modulation_table_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_modulation_table_enable_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageModulationTableEnableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_modulation_table_enable_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_modulation_table_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_modulation_table_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_modulation_table_count: u32,
                            _p_coverage_modulation_table: *const f32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_modulation_table_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageModulationTableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_modulation_table_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_shading_rate_image_enable_nv: unsafe {
                        unsafe extern "system" fn cmd_set_shading_rate_image_enable_nv(
                            _command_buffer: CommandBuffer,
                            _shading_rate_image_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_shading_rate_image_enable_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetShadingRateImageEnableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_shading_rate_image_enable_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_representative_fragment_test_enable_nv: unsafe {
                        unsafe extern "system" fn cmd_set_representative_fragment_test_enable_nv(
                            _command_buffer: CommandBuffer,
                            _representative_fragment_test_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_representative_fragment_test_enable_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetRepresentativeFragmentTestEnableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_representative_fragment_test_enable_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_reduction_mode_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_reduction_mode_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_reduction_mode: CoverageReductionModeNV,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_reduction_mode_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageReductionModeNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_reduction_mode_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_subpass_merge_feedback"]
    pub mod subpass_merge_feedback {
        use super::super::*;
        pub use {
            crate::vk::EXT_SUBPASS_MERGE_FEEDBACK_NAME as NAME,
            crate::vk::EXT_SUBPASS_MERGE_FEEDBACK_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_shader_module_identifier"]
    pub mod shader_module_identifier {
        use super::super::*;
        pub use {
            crate::vk::EXT_SHADER_MODULE_IDENTIFIER_NAME as NAME,
            crate::vk::EXT_SHADER_MODULE_IDENTIFIER_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_shader_module_identifier device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_shader_module_identifier device-level function pointers"]
        pub struct DeviceFn {
            pub get_shader_module_identifier_ext: PFN_vkGetShaderModuleIdentifierEXT,
            pub get_shader_module_create_info_identifier_ext:
                PFN_vkGetShaderModuleCreateInfoIdentifierEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_shader_module_identifier_ext: unsafe {
                        unsafe extern "system" fn get_shader_module_identifier_ext(
                            _device: crate::vk::Device,
                            _shader_module: ShaderModule,
                            _p_identifier: *mut ShaderModuleIdentifierEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_shader_module_identifier_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetShaderModuleIdentifierEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_shader_module_identifier_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_shader_module_create_info_identifier_ext: unsafe {
                        unsafe extern "system" fn get_shader_module_create_info_identifier_ext(
                            _device: crate::vk::Device,
                            _p_create_info: *const ShaderModuleCreateInfo<'_>,
                            _p_identifier: *mut ShaderModuleIdentifierEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_shader_module_create_info_identifier_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetShaderModuleCreateInfoIdentifierEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_shader_module_create_info_identifier_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_rasterization_order_attachment_access"]
    pub mod rasterization_order_attachment_access {
        use super::super::*;
        pub use {
            crate::vk::EXT_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_NAME as NAME,
            crate::vk::EXT_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_legacy_dithering"]
    pub mod legacy_dithering {
        use super::super::*;
        pub use {
            crate::vk::EXT_LEGACY_DITHERING_NAME as NAME,
            crate::vk::EXT_LEGACY_DITHERING_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_pipeline_protected_access"]
    pub mod pipeline_protected_access {
        use super::super::*;
        pub use {
            crate::vk::EXT_PIPELINE_PROTECTED_ACCESS_NAME as NAME,
            crate::vk::EXT_PIPELINE_PROTECTED_ACCESS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_shader_object"]
    pub mod shader_object {
        use super::super::*;
        pub use {
            crate::vk::EXT_SHADER_OBJECT_NAME as NAME,
            crate::vk::EXT_SHADER_OBJECT_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_shader_object device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_shader_object device-level function pointers"]
        pub struct DeviceFn {
            pub create_shaders_ext: PFN_vkCreateShadersEXT,
            pub destroy_shader_ext: PFN_vkDestroyShaderEXT,
            pub get_shader_binary_data_ext: PFN_vkGetShaderBinaryDataEXT,
            pub cmd_bind_shaders_ext: PFN_vkCmdBindShadersEXT,
            pub cmd_set_cull_mode_ext: PFN_vkCmdSetCullMode,
            pub cmd_set_front_face_ext: PFN_vkCmdSetFrontFace,
            pub cmd_set_primitive_topology_ext: PFN_vkCmdSetPrimitiveTopology,
            pub cmd_set_viewport_with_count_ext: PFN_vkCmdSetViewportWithCount,
            pub cmd_set_scissor_with_count_ext: PFN_vkCmdSetScissorWithCount,
            pub cmd_bind_vertex_buffers2_ext: PFN_vkCmdBindVertexBuffers2,
            pub cmd_set_depth_test_enable_ext: PFN_vkCmdSetDepthTestEnable,
            pub cmd_set_depth_write_enable_ext: PFN_vkCmdSetDepthWriteEnable,
            pub cmd_set_depth_compare_op_ext: PFN_vkCmdSetDepthCompareOp,
            pub cmd_set_depth_bounds_test_enable_ext: PFN_vkCmdSetDepthBoundsTestEnable,
            pub cmd_set_stencil_test_enable_ext: PFN_vkCmdSetStencilTestEnable,
            pub cmd_set_stencil_op_ext: PFN_vkCmdSetStencilOp,
            pub cmd_set_vertex_input_ext: PFN_vkCmdSetVertexInputEXT,
            pub cmd_set_patch_control_points_ext: PFN_vkCmdSetPatchControlPointsEXT,
            pub cmd_set_rasterizer_discard_enable_ext: PFN_vkCmdSetRasterizerDiscardEnable,
            pub cmd_set_depth_bias_enable_ext: PFN_vkCmdSetDepthBiasEnable,
            pub cmd_set_logic_op_ext: PFN_vkCmdSetLogicOpEXT,
            pub cmd_set_primitive_restart_enable_ext: PFN_vkCmdSetPrimitiveRestartEnable,
            pub cmd_set_tessellation_domain_origin_ext: PFN_vkCmdSetTessellationDomainOriginEXT,
            pub cmd_set_depth_clamp_enable_ext: PFN_vkCmdSetDepthClampEnableEXT,
            pub cmd_set_polygon_mode_ext: PFN_vkCmdSetPolygonModeEXT,
            pub cmd_set_rasterization_samples_ext: PFN_vkCmdSetRasterizationSamplesEXT,
            pub cmd_set_sample_mask_ext: PFN_vkCmdSetSampleMaskEXT,
            pub cmd_set_alpha_to_coverage_enable_ext: PFN_vkCmdSetAlphaToCoverageEnableEXT,
            pub cmd_set_alpha_to_one_enable_ext: PFN_vkCmdSetAlphaToOneEnableEXT,
            pub cmd_set_logic_op_enable_ext: PFN_vkCmdSetLogicOpEnableEXT,
            pub cmd_set_color_blend_enable_ext: PFN_vkCmdSetColorBlendEnableEXT,
            pub cmd_set_color_blend_equation_ext: PFN_vkCmdSetColorBlendEquationEXT,
            pub cmd_set_color_write_mask_ext: PFN_vkCmdSetColorWriteMaskEXT,
            pub cmd_set_rasterization_stream_ext: PFN_vkCmdSetRasterizationStreamEXT,
            pub cmd_set_conservative_rasterization_mode_ext:
                PFN_vkCmdSetConservativeRasterizationModeEXT,
            pub cmd_set_extra_primitive_overestimation_size_ext:
                PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT,
            pub cmd_set_depth_clip_enable_ext: PFN_vkCmdSetDepthClipEnableEXT,
            pub cmd_set_sample_locations_enable_ext: PFN_vkCmdSetSampleLocationsEnableEXT,
            pub cmd_set_color_blend_advanced_ext: PFN_vkCmdSetColorBlendAdvancedEXT,
            pub cmd_set_provoking_vertex_mode_ext: PFN_vkCmdSetProvokingVertexModeEXT,
            pub cmd_set_line_rasterization_mode_ext: PFN_vkCmdSetLineRasterizationModeEXT,
            pub cmd_set_line_stipple_enable_ext: PFN_vkCmdSetLineStippleEnableEXT,
            pub cmd_set_depth_clip_negative_one_to_one_ext:
                PFN_vkCmdSetDepthClipNegativeOneToOneEXT,
            pub cmd_set_viewport_w_scaling_enable_nv: PFN_vkCmdSetViewportWScalingEnableNV,
            pub cmd_set_viewport_swizzle_nv: PFN_vkCmdSetViewportSwizzleNV,
            pub cmd_set_coverage_to_color_enable_nv: PFN_vkCmdSetCoverageToColorEnableNV,
            pub cmd_set_coverage_to_color_location_nv: PFN_vkCmdSetCoverageToColorLocationNV,
            pub cmd_set_coverage_modulation_mode_nv: PFN_vkCmdSetCoverageModulationModeNV,
            pub cmd_set_coverage_modulation_table_enable_nv:
                PFN_vkCmdSetCoverageModulationTableEnableNV,
            pub cmd_set_coverage_modulation_table_nv: PFN_vkCmdSetCoverageModulationTableNV,
            pub cmd_set_shading_rate_image_enable_nv: PFN_vkCmdSetShadingRateImageEnableNV,
            pub cmd_set_representative_fragment_test_enable_nv:
                PFN_vkCmdSetRepresentativeFragmentTestEnableNV,
            pub cmd_set_coverage_reduction_mode_nv: PFN_vkCmdSetCoverageReductionModeNV,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_shaders_ext: unsafe {
                        unsafe extern "system" fn create_shaders_ext(
                            _device: crate::vk::Device,
                            _create_info_count: u32,
                            _p_create_infos: *const ShaderCreateInfoEXT<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_shaders: *mut ShaderEXT,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(create_shaders_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateShadersEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_shaders_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_shader_ext: unsafe {
                        unsafe extern "system" fn destroy_shader_ext(
                            _device: crate::vk::Device,
                            _shader: ShaderEXT,
                            _p_allocator: *const AllocationCallbacks<'_>,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(destroy_shader_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyShaderEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_shader_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_shader_binary_data_ext: unsafe {
                        unsafe extern "system" fn get_shader_binary_data_ext(
                            _device: crate::vk::Device,
                            _shader: ShaderEXT,
                            _p_data_size: *mut usize,
                            _p_data: *mut c_void,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_shader_binary_data_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetShaderBinaryDataEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_shader_binary_data_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_bind_shaders_ext: unsafe {
                        unsafe extern "system" fn cmd_bind_shaders_ext(
                            _command_buffer: CommandBuffer,
                            _stage_count: u32,
                            _p_stages: *const ShaderStageFlags,
                            _p_shaders: *const ShaderEXT,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_bind_shaders_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdBindShadersEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_bind_shaders_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_cull_mode_ext: unsafe {
                        unsafe extern "system" fn cmd_set_cull_mode_ext(
                            _command_buffer: CommandBuffer,
                            _cull_mode: CullModeFlags,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_cull_mode_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetCullModeEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_cull_mode_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_front_face_ext: unsafe {
                        unsafe extern "system" fn cmd_set_front_face_ext(
                            _command_buffer: CommandBuffer,
                            _front_face: FrontFace,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_front_face_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetFrontFaceEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_front_face_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_primitive_topology_ext: unsafe {
                        unsafe extern "system" fn cmd_set_primitive_topology_ext(
                            _command_buffer: CommandBuffer,
                            _primitive_topology: PrimitiveTopology,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_primitive_topology_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetPrimitiveTopologyEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_primitive_topology_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_viewport_with_count_ext: unsafe {
                        unsafe extern "system" fn cmd_set_viewport_with_count_ext(
                            _command_buffer: CommandBuffer,
                            _viewport_count: u32,
                            _p_viewports: *const Viewport,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_viewport_with_count_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetViewportWithCountEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_viewport_with_count_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_scissor_with_count_ext: unsafe {
                        unsafe extern "system" fn cmd_set_scissor_with_count_ext(
                            _command_buffer: CommandBuffer,
                            _scissor_count: u32,
                            _p_scissors: *const Rect2D,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_scissor_with_count_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetScissorWithCountEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_scissor_with_count_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_bind_vertex_buffers2_ext: unsafe {
                        unsafe extern "system" fn cmd_bind_vertex_buffers2_ext(
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
                                stringify!(cmd_bind_vertex_buffers2_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdBindVertexBuffers2EXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_bind_vertex_buffers2_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_test_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_test_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_test_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_test_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthTestEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_test_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_write_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_write_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_write_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_write_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthWriteEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_write_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_compare_op_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_compare_op_ext(
                            _command_buffer: CommandBuffer,
                            _depth_compare_op: CompareOp,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_compare_op_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthCompareOpEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_compare_op_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_bounds_test_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_bounds_test_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_bounds_test_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_bounds_test_enable_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetDepthBoundsTestEnableEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_bounds_test_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_stencil_test_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_stencil_test_enable_ext(
                            _command_buffer: CommandBuffer,
                            _stencil_test_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_stencil_test_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilTestEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_stencil_test_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_stencil_op_ext: unsafe {
                        unsafe extern "system" fn cmd_set_stencil_op_ext(
                            _command_buffer: CommandBuffer,
                            _face_mask: StencilFaceFlags,
                            _fail_op: StencilOp,
                            _pass_op: StencilOp,
                            _depth_fail_op: StencilOp,
                            _compare_op: CompareOp,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_stencil_op_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilOpEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_stencil_op_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_vertex_input_ext: unsafe {
                        unsafe extern "system" fn cmd_set_vertex_input_ext(
                            _command_buffer: CommandBuffer,
                            _vertex_binding_description_count: u32,
                            _p_vertex_binding_descriptions : * const VertexInputBindingDescription2EXT < '_ >,
                            _vertex_attribute_description_count: u32,
                            _p_vertex_attribute_descriptions : * const VertexInputAttributeDescription2EXT < '_ >,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_vertex_input_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetVertexInputEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_vertex_input_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_patch_control_points_ext: unsafe {
                        unsafe extern "system" fn cmd_set_patch_control_points_ext(
                            _command_buffer: CommandBuffer,
                            _patch_control_points: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_patch_control_points_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetPatchControlPointsEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_patch_control_points_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_rasterizer_discard_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_rasterizer_discard_enable_ext(
                            _command_buffer: CommandBuffer,
                            _rasterizer_discard_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_rasterizer_discard_enable_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetRasterizerDiscardEnableEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_rasterizer_discard_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_bias_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_bias_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_bias_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_bias_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthBiasEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_bias_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_logic_op_ext: unsafe {
                        unsafe extern "system" fn cmd_set_logic_op_ext(
                            _command_buffer: CommandBuffer,
                            _logic_op: LogicOp,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_set_logic_op_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLogicOpEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_logic_op_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_primitive_restart_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_primitive_restart_enable_ext(
                            _command_buffer: CommandBuffer,
                            _primitive_restart_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_primitive_restart_enable_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetPrimitiveRestartEnableEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_primitive_restart_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_tessellation_domain_origin_ext: unsafe {
                        unsafe extern "system" fn cmd_set_tessellation_domain_origin_ext(
                            _command_buffer: CommandBuffer,
                            _domain_origin: TessellationDomainOrigin,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_tessellation_domain_origin_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetTessellationDomainOriginEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_tessellation_domain_origin_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_clamp_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_clamp_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_clamp_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_clamp_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthClampEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_clamp_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_polygon_mode_ext: unsafe {
                        unsafe extern "system" fn cmd_set_polygon_mode_ext(
                            _command_buffer: CommandBuffer,
                            _polygon_mode: PolygonMode,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_polygon_mode_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetPolygonModeEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_polygon_mode_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_rasterization_samples_ext: unsafe {
                        unsafe extern "system" fn cmd_set_rasterization_samples_ext(
                            _command_buffer: CommandBuffer,
                            _rasterization_samples: SampleCountFlags,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_rasterization_samples_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetRasterizationSamplesEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_rasterization_samples_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_sample_mask_ext: unsafe {
                        unsafe extern "system" fn cmd_set_sample_mask_ext(
                            _command_buffer: CommandBuffer,
                            _samples: SampleCountFlags,
                            _p_sample_mask: *const SampleMask,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_sample_mask_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetSampleMaskEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_sample_mask_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_alpha_to_coverage_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_alpha_to_coverage_enable_ext(
                            _command_buffer: CommandBuffer,
                            _alpha_to_coverage_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_alpha_to_coverage_enable_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetAlphaToCoverageEnableEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_alpha_to_coverage_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_alpha_to_one_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_alpha_to_one_enable_ext(
                            _command_buffer: CommandBuffer,
                            _alpha_to_one_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_alpha_to_one_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetAlphaToOneEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_alpha_to_one_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_logic_op_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_logic_op_enable_ext(
                            _command_buffer: CommandBuffer,
                            _logic_op_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_logic_op_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLogicOpEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_logic_op_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_color_blend_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_color_blend_enable_ext(
                            _command_buffer: CommandBuffer,
                            _first_attachment: u32,
                            _attachment_count: u32,
                            _p_color_blend_enables: *const Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_color_blend_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetColorBlendEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_color_blend_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_color_blend_equation_ext: unsafe {
                        unsafe extern "system" fn cmd_set_color_blend_equation_ext(
                            _command_buffer: CommandBuffer,
                            _first_attachment: u32,
                            _attachment_count: u32,
                            _p_color_blend_equations: *const ColorBlendEquationEXT,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_color_blend_equation_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetColorBlendEquationEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_color_blend_equation_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_color_write_mask_ext: unsafe {
                        unsafe extern "system" fn cmd_set_color_write_mask_ext(
                            _command_buffer: CommandBuffer,
                            _first_attachment: u32,
                            _attachment_count: u32,
                            _p_color_write_masks: *const ColorComponentFlags,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_color_write_mask_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetColorWriteMaskEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_color_write_mask_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_rasterization_stream_ext: unsafe {
                        unsafe extern "system" fn cmd_set_rasterization_stream_ext(
                            _command_buffer: CommandBuffer,
                            _rasterization_stream: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_rasterization_stream_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetRasterizationStreamEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_rasterization_stream_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_conservative_rasterization_mode_ext: unsafe {
                        unsafe extern "system" fn cmd_set_conservative_rasterization_mode_ext(
                            _command_buffer: CommandBuffer,
                            _conservative_rasterization_mode: ConservativeRasterizationModeEXT,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_conservative_rasterization_mode_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetConservativeRasterizationModeEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_conservative_rasterization_mode_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_extra_primitive_overestimation_size_ext: unsafe {
                        unsafe extern "system" fn cmd_set_extra_primitive_overestimation_size_ext(
                            _command_buffer: CommandBuffer,
                            _extra_primitive_overestimation_size: f32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_extra_primitive_overestimation_size_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetExtraPrimitiveOverestimationSizeEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_extra_primitive_overestimation_size_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_clip_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_clip_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_clip_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_clip_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthClipEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_clip_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_sample_locations_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_sample_locations_enable_ext(
                            _command_buffer: CommandBuffer,
                            _sample_locations_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_sample_locations_enable_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetSampleLocationsEnableEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_sample_locations_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_color_blend_advanced_ext: unsafe {
                        unsafe extern "system" fn cmd_set_color_blend_advanced_ext(
                            _command_buffer: CommandBuffer,
                            _first_attachment: u32,
                            _attachment_count: u32,
                            _p_color_blend_advanced: *const ColorBlendAdvancedEXT,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_color_blend_advanced_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetColorBlendAdvancedEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_color_blend_advanced_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_provoking_vertex_mode_ext: unsafe {
                        unsafe extern "system" fn cmd_set_provoking_vertex_mode_ext(
                            _command_buffer: CommandBuffer,
                            _provoking_vertex_mode: ProvokingVertexModeEXT,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_provoking_vertex_mode_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetProvokingVertexModeEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_provoking_vertex_mode_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_line_rasterization_mode_ext: unsafe {
                        unsafe extern "system" fn cmd_set_line_rasterization_mode_ext(
                            _command_buffer: CommandBuffer,
                            _line_rasterization_mode: LineRasterizationModeEXT,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_line_rasterization_mode_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetLineRasterizationModeEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_line_rasterization_mode_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_line_stipple_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_line_stipple_enable_ext(
                            _command_buffer: CommandBuffer,
                            _stippled_line_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_line_stipple_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLineStippleEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_line_stipple_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_clip_negative_one_to_one_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_clip_negative_one_to_one_ext(
                            _command_buffer: CommandBuffer,
                            _negative_one_to_one: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_clip_negative_one_to_one_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetDepthClipNegativeOneToOneEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_clip_negative_one_to_one_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_viewport_w_scaling_enable_nv: unsafe {
                        unsafe extern "system" fn cmd_set_viewport_w_scaling_enable_nv(
                            _command_buffer: CommandBuffer,
                            _viewport_w_scaling_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_viewport_w_scaling_enable_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetViewportWScalingEnableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_viewport_w_scaling_enable_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_viewport_swizzle_nv: unsafe {
                        unsafe extern "system" fn cmd_set_viewport_swizzle_nv(
                            _command_buffer: CommandBuffer,
                            _first_viewport: u32,
                            _viewport_count: u32,
                            _p_viewport_swizzles: *const ViewportSwizzleNV,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_viewport_swizzle_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetViewportSwizzleNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_viewport_swizzle_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_to_color_enable_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_to_color_enable_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_to_color_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_to_color_enable_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageToColorEnableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_to_color_enable_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_to_color_location_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_to_color_location_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_to_color_location: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_to_color_location_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageToColorLocationNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_to_color_location_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_modulation_mode_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_modulation_mode_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_modulation_mode: CoverageModulationModeNV,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_modulation_mode_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageModulationModeNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_modulation_mode_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_modulation_table_enable_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_modulation_table_enable_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_modulation_table_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_modulation_table_enable_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageModulationTableEnableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_modulation_table_enable_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_modulation_table_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_modulation_table_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_modulation_table_count: u32,
                            _p_coverage_modulation_table: *const f32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_modulation_table_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageModulationTableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_modulation_table_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_shading_rate_image_enable_nv: unsafe {
                        unsafe extern "system" fn cmd_set_shading_rate_image_enable_nv(
                            _command_buffer: CommandBuffer,
                            _shading_rate_image_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_shading_rate_image_enable_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetShadingRateImageEnableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_shading_rate_image_enable_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_representative_fragment_test_enable_nv: unsafe {
                        unsafe extern "system" fn cmd_set_representative_fragment_test_enable_nv(
                            _command_buffer: CommandBuffer,
                            _representative_fragment_test_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_representative_fragment_test_enable_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetRepresentativeFragmentTestEnableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_representative_fragment_test_enable_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_reduction_mode_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_reduction_mode_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_reduction_mode: CoverageReductionModeNV,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_reduction_mode_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageReductionModeNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_reduction_mode_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_mutable_descriptor_type"]
    pub mod mutable_descriptor_type {
        use super::super::*;
        pub use {
            crate::vk::EXT_MUTABLE_DESCRIPTOR_TYPE_NAME as NAME,
            crate::vk::EXT_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_layer_settings"]
    pub mod layer_settings {
        use super::super::*;
        pub use {
            crate::vk::EXT_LAYER_SETTINGS_NAME as NAME,
            crate::vk::EXT_LAYER_SETTINGS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_pipeline_library_group_handles"]
    pub mod pipeline_library_group_handles {
        use super::super::*;
        pub use {
            crate::vk::EXT_PIPELINE_LIBRARY_GROUP_HANDLES_NAME as NAME,
            crate::vk::EXT_PIPELINE_LIBRARY_GROUP_HANDLES_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_dynamic_rendering_unused_attachments"]
    pub mod dynamic_rendering_unused_attachments {
        use super::super::*;
        pub use {
            crate::vk::EXT_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_NAME as NAME,
            crate::vk::EXT_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_attachment_feedback_loop_dynamic_state"]
    pub mod attachment_feedback_loop_dynamic_state {
        use super::super::*;
        pub use {
            crate::vk::EXT_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_NAME as NAME,
            crate::vk::EXT_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_attachment_feedback_loop_dynamic_state device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_attachment_feedback_loop_dynamic_state device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_set_attachment_feedback_loop_enable_ext:
                PFN_vkCmdSetAttachmentFeedbackLoopEnableEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_set_attachment_feedback_loop_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_attachment_feedback_loop_enable_ext(
                            _command_buffer: CommandBuffer,
                            _aspect_mask: ImageAspectFlags,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_attachment_feedback_loop_enable_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetAttachmentFeedbackLoopEnableEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_attachment_feedback_loop_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
}
#[doc = "Extensions tagged FUCHSIA"]
pub mod fuchsia {
    #[doc = "VK_FUCHSIA_imagepipe_surface"]
    pub mod imagepipe_surface {
        use super::super::*;
        pub use {
            crate::vk::FUCHSIA_IMAGEPIPE_SURFACE_NAME as NAME,
            crate::vk::FUCHSIA_IMAGEPIPE_SURFACE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_FUCHSIA_imagepipe_surface instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_FUCHSIA_imagepipe_surface instance-level function pointers"]
        pub struct InstanceFn {
            pub create_image_pipe_surface_fuchsia: PFN_vkCreateImagePipeSurfaceFUCHSIA,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_image_pipe_surface_fuchsia: unsafe {
                        unsafe extern "system" fn create_image_pipe_surface_fuchsia(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const ImagePipeSurfaceCreateInfoFUCHSIA<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_surface: *mut SurfaceKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_image_pipe_surface_fuchsia)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCreateImagePipeSurfaceFUCHSIA\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            create_image_pipe_surface_fuchsia
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_FUCHSIA_external_memory"]
    pub mod external_memory {
        use super::super::*;
        pub use {
            crate::vk::FUCHSIA_EXTERNAL_MEMORY_NAME as NAME,
            crate::vk::FUCHSIA_EXTERNAL_MEMORY_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_FUCHSIA_external_memory device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_FUCHSIA_external_memory device-level function pointers"]
        pub struct DeviceFn {
            pub get_memory_zircon_handle_fuchsia: PFN_vkGetMemoryZirconHandleFUCHSIA,
            pub get_memory_zircon_handle_properties_fuchsia:
                PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_memory_zircon_handle_fuchsia: unsafe {
                        unsafe extern "system" fn get_memory_zircon_handle_fuchsia(
                            _device: crate::vk::Device,
                            _p_get_zircon_handle_info: *const MemoryGetZirconHandleInfoFUCHSIA<'_>,
                            _p_zircon_handle: *mut zx_handle_t,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_memory_zircon_handle_fuchsia)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetMemoryZirconHandleFUCHSIA\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_memory_zircon_handle_fuchsia
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_memory_zircon_handle_properties_fuchsia: unsafe {
                        unsafe extern "system" fn get_memory_zircon_handle_properties_fuchsia(
                            _device: crate::vk::Device,
                            _handle_type: ExternalMemoryHandleTypeFlags,
                            _zircon_handle: zx_handle_t,
                            _p_memory_zircon_handle_properties : * mut MemoryZirconHandlePropertiesFUCHSIA < '_ >,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_memory_zircon_handle_properties_fuchsia)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetMemoryZirconHandlePropertiesFUCHSIA\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_memory_zircon_handle_properties_fuchsia
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_FUCHSIA_external_semaphore"]
    pub mod external_semaphore {
        use super::super::*;
        pub use {
            crate::vk::FUCHSIA_EXTERNAL_SEMAPHORE_NAME as NAME,
            crate::vk::FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_FUCHSIA_external_semaphore device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_FUCHSIA_external_semaphore device-level function pointers"]
        pub struct DeviceFn {
            pub import_semaphore_zircon_handle_fuchsia: PFN_vkImportSemaphoreZirconHandleFUCHSIA,
            pub get_semaphore_zircon_handle_fuchsia: PFN_vkGetSemaphoreZirconHandleFUCHSIA,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    import_semaphore_zircon_handle_fuchsia: unsafe {
                        unsafe extern "system" fn import_semaphore_zircon_handle_fuchsia(
                            _device: crate::vk::Device,
                            _p_import_semaphore_zircon_handle_info : * const ImportSemaphoreZirconHandleInfoFUCHSIA < '_ >,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(import_semaphore_zircon_handle_fuchsia)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkImportSemaphoreZirconHandleFUCHSIA\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            import_semaphore_zircon_handle_fuchsia
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_semaphore_zircon_handle_fuchsia: unsafe {
                        unsafe extern "system" fn get_semaphore_zircon_handle_fuchsia(
                            _device: crate::vk::Device,
                            _p_get_zircon_handle_info: *const SemaphoreGetZirconHandleInfoFUCHSIA<
                                '_,
                            >,
                            _p_zircon_handle: *mut zx_handle_t,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_semaphore_zircon_handle_fuchsia)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetSemaphoreZirconHandleFUCHSIA\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_semaphore_zircon_handle_fuchsia
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_FUCHSIA_buffer_collection"]
    pub mod buffer_collection {
        use super::super::*;
        pub use {
            crate::vk::FUCHSIA_BUFFER_COLLECTION_NAME as NAME,
            crate::vk::FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_FUCHSIA_buffer_collection device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_FUCHSIA_buffer_collection device-level function pointers"]
        pub struct DeviceFn {
            pub create_buffer_collection_fuchsia: PFN_vkCreateBufferCollectionFUCHSIA,
            pub set_buffer_collection_image_constraints_fuchsia:
                PFN_vkSetBufferCollectionImageConstraintsFUCHSIA,
            pub set_buffer_collection_buffer_constraints_fuchsia:
                PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA,
            pub destroy_buffer_collection_fuchsia: PFN_vkDestroyBufferCollectionFUCHSIA,
            pub get_buffer_collection_properties_fuchsia:
                PFN_vkGetBufferCollectionPropertiesFUCHSIA,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_buffer_collection_fuchsia: unsafe {
                        unsafe extern "system" fn create_buffer_collection_fuchsia(
                            _device: crate::vk::Device,
                            _p_create_info: *const BufferCollectionCreateInfoFUCHSIA<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_collection: *mut BufferCollectionFUCHSIA,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_buffer_collection_fuchsia)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCreateBufferCollectionFUCHSIA\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            create_buffer_collection_fuchsia
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    set_buffer_collection_image_constraints_fuchsia: unsafe {
                        unsafe extern "system" fn set_buffer_collection_image_constraints_fuchsia(
                            _device: crate::vk::Device,
                            _collection: BufferCollectionFUCHSIA,
                            _p_image_constraints_info: *const ImageConstraintsInfoFUCHSIA<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(set_buffer_collection_image_constraints_fuchsia)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkSetBufferCollectionImageConstraintsFUCHSIA\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            set_buffer_collection_image_constraints_fuchsia
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    set_buffer_collection_buffer_constraints_fuchsia: unsafe {
                        unsafe extern "system" fn set_buffer_collection_buffer_constraints_fuchsia(
                            _device: crate::vk::Device,
                            _collection: BufferCollectionFUCHSIA,
                            _p_buffer_constraints_info: *const BufferConstraintsInfoFUCHSIA<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(set_buffer_collection_buffer_constraints_fuchsia)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkSetBufferCollectionBufferConstraintsFUCHSIA\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            set_buffer_collection_buffer_constraints_fuchsia
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_buffer_collection_fuchsia: unsafe {
                        unsafe extern "system" fn destroy_buffer_collection_fuchsia(
                            _device: crate::vk::Device,
                            _collection: BufferCollectionFUCHSIA,
                            _p_allocator: *const AllocationCallbacks<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(destroy_buffer_collection_fuchsia)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkDestroyBufferCollectionFUCHSIA\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_buffer_collection_fuchsia
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_buffer_collection_properties_fuchsia: unsafe {
                        unsafe extern "system" fn get_buffer_collection_properties_fuchsia(
                            _device: crate::vk::Device,
                            _collection: BufferCollectionFUCHSIA,
                            _p_properties: *mut BufferCollectionPropertiesFUCHSIA<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_buffer_collection_properties_fuchsia)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetBufferCollectionPropertiesFUCHSIA\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_buffer_collection_properties_fuchsia
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
}
#[doc = "Extensions tagged GGP"]
pub mod ggp {
    #[doc = "VK_GGP_stream_descriptor_surface"]
    pub mod stream_descriptor_surface {
        use super::super::*;
        pub use {
            crate::vk::GGP_STREAM_DESCRIPTOR_SURFACE_NAME as NAME,
            crate::vk::GGP_STREAM_DESCRIPTOR_SURFACE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_GGP_stream_descriptor_surface instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_GGP_stream_descriptor_surface instance-level function pointers"]
        pub struct InstanceFn {
            pub create_stream_descriptor_surface_ggp: PFN_vkCreateStreamDescriptorSurfaceGGP,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_stream_descriptor_surface_ggp: unsafe {
                        unsafe extern "system" fn create_stream_descriptor_surface_ggp(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const StreamDescriptorSurfaceCreateInfoGGP<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_surface: *mut SurfaceKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_stream_descriptor_surface_ggp)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCreateStreamDescriptorSurfaceGGP\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            create_stream_descriptor_surface_ggp
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_GGP_frame_token"]
    pub mod frame_token {
        use super::super::*;
        pub use {
            crate::vk::GGP_FRAME_TOKEN_NAME as NAME,
            crate::vk::GGP_FRAME_TOKEN_SPEC_VERSION as SPEC_VERSION,
        };
    }
}
#[doc = "Extensions tagged GOOGLE"]
pub mod google {
    #[doc = "VK_GOOGLE_display_timing"]
    pub mod display_timing {
        use super::super::*;
        pub use {
            crate::vk::GOOGLE_DISPLAY_TIMING_NAME as NAME,
            crate::vk::GOOGLE_DISPLAY_TIMING_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_GOOGLE_display_timing device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_GOOGLE_display_timing device-level function pointers"]
        pub struct DeviceFn {
            pub get_refresh_cycle_duration_google: PFN_vkGetRefreshCycleDurationGOOGLE,
            pub get_past_presentation_timing_google: PFN_vkGetPastPresentationTimingGOOGLE,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_refresh_cycle_duration_google: unsafe {
                        unsafe extern "system" fn get_refresh_cycle_duration_google(
                            _device: crate::vk::Device,
                            _swapchain: SwapchainKHR,
                            _p_display_timing_properties: *mut RefreshCycleDurationGOOGLE,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_refresh_cycle_duration_google)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetRefreshCycleDurationGOOGLE\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_refresh_cycle_duration_google
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_past_presentation_timing_google: unsafe {
                        unsafe extern "system" fn get_past_presentation_timing_google(
                            _device: crate::vk::Device,
                            _swapchain: SwapchainKHR,
                            _p_presentation_timing_count: *mut u32,
                            _p_presentation_timings: *mut PastPresentationTimingGOOGLE,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_past_presentation_timing_google)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPastPresentationTimingGOOGLE\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_past_presentation_timing_google
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_GOOGLE_hlsl_functionality1"]
    pub mod hlsl_functionality1 {
        use super::super::*;
        pub use {
            crate::vk::GOOGLE_HLSL_FUNCTIONALITY1_NAME as NAME,
            crate::vk::GOOGLE_HLSL_FUNCTIONALITY1_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_GOOGLE_decorate_string"]
    pub mod decorate_string {
        use super::super::*;
        pub use {
            crate::vk::GOOGLE_DECORATE_STRING_NAME as NAME,
            crate::vk::GOOGLE_DECORATE_STRING_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_GOOGLE_user_type"]
    pub mod user_type {
        use super::super::*;
        pub use {
            crate::vk::GOOGLE_USER_TYPE_NAME as NAME,
            crate::vk::GOOGLE_USER_TYPE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_GOOGLE_surfaceless_query"]
    pub mod surfaceless_query {
        use super::super::*;
        pub use {
            crate::vk::GOOGLE_SURFACELESS_QUERY_NAME as NAME,
            crate::vk::GOOGLE_SURFACELESS_QUERY_SPEC_VERSION as SPEC_VERSION,
        };
    }
}
#[doc = "Extensions tagged HUAWEI"]
pub mod huawei {
    #[doc = "VK_HUAWEI_subpass_shading"]
    pub mod subpass_shading {
        use super::super::*;
        pub use {
            crate::vk::HUAWEI_SUBPASS_SHADING_NAME as NAME,
            crate::vk::HUAWEI_SUBPASS_SHADING_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_HUAWEI_subpass_shading device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_HUAWEI_subpass_shading device-level function pointers"]
        pub struct DeviceFn {
            pub get_device_subpass_shading_max_workgroup_size_huawei:
                PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI,
            pub cmd_subpass_shading_huawei: PFN_vkCmdSubpassShadingHUAWEI,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_device_subpass_shading_max_workgroup_size_huawei: unsafe {
                        unsafe extern "system" fn get_device_subpass_shading_max_workgroup_size_huawei(
                            _device: crate::vk::Device,
                            _renderpass: RenderPass,
                            _p_max_workgroup_size: *mut Extent2D,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_device_subpass_shading_max_workgroup_size_huawei)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_device_subpass_shading_max_workgroup_size_huawei
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_subpass_shading_huawei: unsafe {
                        unsafe extern "system" fn cmd_subpass_shading_huawei(
                            _command_buffer: CommandBuffer,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_subpass_shading_huawei)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSubpassShadingHUAWEI\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_subpass_shading_huawei
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_HUAWEI_invocation_mask"]
    pub mod invocation_mask {
        use super::super::*;
        pub use {
            crate::vk::HUAWEI_INVOCATION_MASK_NAME as NAME,
            crate::vk::HUAWEI_INVOCATION_MASK_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_HUAWEI_invocation_mask device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_HUAWEI_invocation_mask device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_bind_invocation_mask_huawei: PFN_vkCmdBindInvocationMaskHUAWEI,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_bind_invocation_mask_huawei: unsafe {
                        unsafe extern "system" fn cmd_bind_invocation_mask_huawei(
                            _command_buffer: CommandBuffer,
                            _image_view: ImageView,
                            _image_layout: ImageLayout,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_bind_invocation_mask_huawei)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdBindInvocationMaskHUAWEI\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_bind_invocation_mask_huawei
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_HUAWEI_cluster_culling_shader"]
    pub mod cluster_culling_shader {
        use super::super::*;
        pub use {
            crate::vk::HUAWEI_CLUSTER_CULLING_SHADER_NAME as NAME,
            crate::vk::HUAWEI_CLUSTER_CULLING_SHADER_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_HUAWEI_cluster_culling_shader device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_HUAWEI_cluster_culling_shader device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_draw_cluster_huawei: PFN_vkCmdDrawClusterHUAWEI,
            pub cmd_draw_cluster_indirect_huawei: PFN_vkCmdDrawClusterIndirectHUAWEI,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_draw_cluster_huawei: unsafe {
                        unsafe extern "system" fn cmd_draw_cluster_huawei(
                            _command_buffer: CommandBuffer,
                            _group_count_x: u32,
                            _group_count_y: u32,
                            _group_count_z: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_draw_cluster_huawei)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawClusterHUAWEI\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_draw_cluster_huawei
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_draw_cluster_indirect_huawei: unsafe {
                        unsafe extern "system" fn cmd_draw_cluster_indirect_huawei(
                            _command_buffer: CommandBuffer,
                            _buffer: Buffer,
                            _offset: DeviceSize,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_draw_cluster_indirect_huawei)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdDrawClusterIndirectHUAWEI\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_draw_cluster_indirect_huawei
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
}
#[doc = "Extensions tagged IMG"]
pub mod img {
    #[doc = "VK_IMG_filter_cubic"]
    pub mod filter_cubic {
        use super::super::*;
        pub use {
            crate::vk::IMG_FILTER_CUBIC_NAME as NAME,
            crate::vk::IMG_FILTER_CUBIC_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_IMG_format_pvrtc"]
    pub mod format_pvrtc {
        use super::super::*;
        pub use {
            crate::vk::IMG_FORMAT_PVRTC_NAME as NAME,
            crate::vk::IMG_FORMAT_PVRTC_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_IMG_relaxed_line_rasterization"]
    pub mod relaxed_line_rasterization {
        use super::super::*;
        pub use {
            crate::vk::IMG_RELAXED_LINE_RASTERIZATION_NAME as NAME,
            crate::vk::IMG_RELAXED_LINE_RASTERIZATION_SPEC_VERSION as SPEC_VERSION,
        };
    }
}
#[doc = "Extensions tagged INTEL"]
pub mod intel {
    #[doc = "VK_INTEL_shader_integer_functions2"]
    pub mod shader_integer_functions2 {
        use super::super::*;
        pub use {
            crate::vk::INTEL_SHADER_INTEGER_FUNCTIONS2_NAME as NAME,
            crate::vk::INTEL_SHADER_INTEGER_FUNCTIONS2_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_INTEL_performance_query"]
    pub mod performance_query {
        use super::super::*;
        pub use {
            crate::vk::INTEL_PERFORMANCE_QUERY_NAME as NAME,
            crate::vk::INTEL_PERFORMANCE_QUERY_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_INTEL_performance_query device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_INTEL_performance_query device-level function pointers"]
        pub struct DeviceFn {
            pub initialize_performance_api_intel: PFN_vkInitializePerformanceApiINTEL,
            pub uninitialize_performance_api_intel: PFN_vkUninitializePerformanceApiINTEL,
            pub cmd_set_performance_marker_intel: PFN_vkCmdSetPerformanceMarkerINTEL,
            pub cmd_set_performance_stream_marker_intel: PFN_vkCmdSetPerformanceStreamMarkerINTEL,
            pub cmd_set_performance_override_intel: PFN_vkCmdSetPerformanceOverrideINTEL,
            pub acquire_performance_configuration_intel: PFN_vkAcquirePerformanceConfigurationINTEL,
            pub release_performance_configuration_intel: PFN_vkReleasePerformanceConfigurationINTEL,
            pub queue_set_performance_configuration_intel:
                PFN_vkQueueSetPerformanceConfigurationINTEL,
            pub get_performance_parameter_intel: PFN_vkGetPerformanceParameterINTEL,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    initialize_performance_api_intel: unsafe {
                        unsafe extern "system" fn initialize_performance_api_intel(
                            _device: crate::vk::Device,
                            _p_initialize_info: *const InitializePerformanceApiInfoINTEL<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(initialize_performance_api_intel)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkInitializePerformanceApiINTEL\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            initialize_performance_api_intel
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    uninitialize_performance_api_intel: unsafe {
                        unsafe extern "system" fn uninitialize_performance_api_intel(
                            _device: crate::vk::Device,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(uninitialize_performance_api_intel)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkUninitializePerformanceApiINTEL\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            uninitialize_performance_api_intel
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_performance_marker_intel: unsafe {
                        unsafe extern "system" fn cmd_set_performance_marker_intel(
                            _command_buffer: CommandBuffer,
                            _p_marker_info: *const PerformanceMarkerInfoINTEL<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_performance_marker_intel)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetPerformanceMarkerINTEL\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_performance_marker_intel
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_performance_stream_marker_intel: unsafe {
                        unsafe extern "system" fn cmd_set_performance_stream_marker_intel(
                            _command_buffer: CommandBuffer,
                            _p_marker_info: *const PerformanceStreamMarkerInfoINTEL<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_performance_stream_marker_intel)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetPerformanceStreamMarkerINTEL\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_performance_stream_marker_intel
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_performance_override_intel: unsafe {
                        unsafe extern "system" fn cmd_set_performance_override_intel(
                            _command_buffer: CommandBuffer,
                            _p_override_info: *const PerformanceOverrideInfoINTEL<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_performance_override_intel)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetPerformanceOverrideINTEL\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_performance_override_intel
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    acquire_performance_configuration_intel: unsafe {
                        unsafe extern "system" fn acquire_performance_configuration_intel(
                            _device: crate::vk::Device,
                            _p_acquire_info: *const PerformanceConfigurationAcquireInfoINTEL<'_>,
                            _p_configuration: *mut PerformanceConfigurationINTEL,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(acquire_performance_configuration_intel)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkAcquirePerformanceConfigurationINTEL\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            acquire_performance_configuration_intel
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    release_performance_configuration_intel: unsafe {
                        unsafe extern "system" fn release_performance_configuration_intel(
                            _device: crate::vk::Device,
                            _configuration: PerformanceConfigurationINTEL,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(release_performance_configuration_intel)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkReleasePerformanceConfigurationINTEL\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            release_performance_configuration_intel
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    queue_set_performance_configuration_intel: unsafe {
                        unsafe extern "system" fn queue_set_performance_configuration_intel(
                            _queue: Queue,
                            _configuration: PerformanceConfigurationINTEL,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(queue_set_performance_configuration_intel)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkQueueSetPerformanceConfigurationINTEL\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            queue_set_performance_configuration_intel
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_performance_parameter_intel: unsafe {
                        unsafe extern "system" fn get_performance_parameter_intel(
                            _device: crate::vk::Device,
                            _parameter: PerformanceParameterTypeINTEL,
                            _p_value: *mut PerformanceValueINTEL,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_performance_parameter_intel)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPerformanceParameterINTEL\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_performance_parameter_intel
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
}
#[doc = "Extensions tagged KHR"]
pub mod khr {
    #[doc = "VK_KHR_surface"]
    pub mod surface {
        use super::super::*;
        pub use {
            crate::vk::KHR_SURFACE_NAME as NAME,
            crate::vk::KHR_SURFACE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_surface instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_surface instance-level function pointers"]
        pub struct InstanceFn {
            pub destroy_surface_khr: PFN_vkDestroySurfaceKHR,
            pub get_physical_device_surface_support_khr: PFN_vkGetPhysicalDeviceSurfaceSupportKHR,
            pub get_physical_device_surface_capabilities_khr:
                PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
            pub get_physical_device_surface_formats_khr: PFN_vkGetPhysicalDeviceSurfaceFormatsKHR,
            pub get_physical_device_surface_present_modes_khr:
                PFN_vkGetPhysicalDeviceSurfacePresentModesKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    destroy_surface_khr: unsafe {
                        unsafe extern "system" fn destroy_surface_khr(
                            _instance: crate::vk::Instance,
                            _surface: SurfaceKHR,
                            _p_allocator: *const AllocationCallbacks<'_>,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(destroy_surface_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroySurfaceKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_surface_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_surface_support_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_surface_support_khr(
                            _physical_device: PhysicalDevice,
                            _queue_family_index: u32,
                            _surface: SurfaceKHR,
                            _p_supported: *mut Bool32,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_surface_support_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceSurfaceSupportKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_surface_support_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_surface_capabilities_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_surface_capabilities_khr(
                            _physical_device: PhysicalDevice,
                            _surface: SurfaceKHR,
                            _p_surface_capabilities: *mut SurfaceCapabilitiesKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_surface_capabilities_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceSurfaceCapabilitiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_surface_capabilities_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_surface_formats_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_surface_formats_khr(
                            _physical_device: PhysicalDevice,
                            _surface: SurfaceKHR,
                            _p_surface_format_count: *mut u32,
                            _p_surface_formats: *mut SurfaceFormatKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_surface_formats_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceSurfaceFormatsKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_surface_formats_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_surface_present_modes_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_surface_present_modes_khr(
                            _physical_device: PhysicalDevice,
                            _surface: SurfaceKHR,
                            _p_present_mode_count: *mut u32,
                            _p_present_modes: *mut PresentModeKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_surface_present_modes_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceSurfacePresentModesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_surface_present_modes_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_swapchain"]
    pub mod swapchain {
        use super::super::*;
        pub use {
            crate::vk::KHR_SWAPCHAIN_NAME as NAME,
            crate::vk::KHR_SWAPCHAIN_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_swapchain instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_swapchain instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_present_rectangles_khr:
                PFN_vkGetPhysicalDevicePresentRectanglesKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_present_rectangles_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_present_rectangles_khr(
                            _physical_device: PhysicalDevice,
                            _surface: SurfaceKHR,
                            _p_rect_count: *mut u32,
                            _p_rects: *mut Rect2D,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_present_rectangles_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDevicePresentRectanglesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_present_rectangles_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
        #[doc = "VK_KHR_swapchain device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_swapchain device-level function pointers"]
        pub struct DeviceFn {
            pub create_swapchain_khr: PFN_vkCreateSwapchainKHR,
            pub destroy_swapchain_khr: PFN_vkDestroySwapchainKHR,
            pub get_swapchain_images_khr: PFN_vkGetSwapchainImagesKHR,
            pub acquire_next_image_khr: PFN_vkAcquireNextImageKHR,
            pub queue_present_khr: PFN_vkQueuePresentKHR,
            pub get_device_group_present_capabilities_khr:
                PFN_vkGetDeviceGroupPresentCapabilitiesKHR,
            pub get_device_group_surface_present_modes_khr:
                PFN_vkGetDeviceGroupSurfacePresentModesKHR,
            pub acquire_next_image2_khr: PFN_vkAcquireNextImage2KHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_swapchain_khr: unsafe {
                        unsafe extern "system" fn create_swapchain_khr(
                            _device: crate::vk::Device,
                            _p_create_info: *const SwapchainCreateInfoKHR<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_swapchain: *mut SwapchainKHR,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(create_swapchain_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateSwapchainKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_swapchain_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_swapchain_khr: unsafe {
                        unsafe extern "system" fn destroy_swapchain_khr(
                            _device: crate::vk::Device,
                            _swapchain: SwapchainKHR,
                            _p_allocator: *const AllocationCallbacks<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(destroy_swapchain_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroySwapchainKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_swapchain_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_swapchain_images_khr: unsafe {
                        unsafe extern "system" fn get_swapchain_images_khr(
                            _device: crate::vk::Device,
                            _swapchain: SwapchainKHR,
                            _p_swapchain_image_count: *mut u32,
                            _p_swapchain_images: *mut Image,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_swapchain_images_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetSwapchainImagesKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_swapchain_images_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    acquire_next_image_khr: unsafe {
                        unsafe extern "system" fn acquire_next_image_khr(
                            _device: crate::vk::Device,
                            _swapchain: SwapchainKHR,
                            _timeout: u64,
                            _semaphore: Semaphore,
                            _fence: Fence,
                            _p_image_index: *mut u32,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(acquire_next_image_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkAcquireNextImageKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            acquire_next_image_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    queue_present_khr: unsafe {
                        unsafe extern "system" fn queue_present_khr(
                            _queue: Queue,
                            _p_present_info: *const PresentInfoKHR<'_>,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(queue_present_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkQueuePresentKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            queue_present_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_device_group_present_capabilities_khr: unsafe {
                        unsafe extern "system" fn get_device_group_present_capabilities_khr(
                            _device: crate::vk::Device,
                            _p_device_group_present_capabilities : * mut DeviceGroupPresentCapabilitiesKHR < '_ >,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_device_group_present_capabilities_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeviceGroupPresentCapabilitiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_device_group_present_capabilities_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_device_group_surface_present_modes_khr: unsafe {
                        unsafe extern "system" fn get_device_group_surface_present_modes_khr(
                            _device: crate::vk::Device,
                            _surface: SurfaceKHR,
                            _p_modes: *mut DeviceGroupPresentModeFlagsKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_device_group_surface_present_modes_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeviceGroupSurfacePresentModesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_device_group_surface_present_modes_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    acquire_next_image2_khr: unsafe {
                        unsafe extern "system" fn acquire_next_image2_khr(
                            _device: crate::vk::Device,
                            _p_acquire_info: *const AcquireNextImageInfoKHR<'_>,
                            _p_image_index: *mut u32,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(acquire_next_image2_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkAcquireNextImage2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            acquire_next_image2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_display"]
    pub mod display {
        use super::super::*;
        pub use {
            crate::vk::KHR_DISPLAY_NAME as NAME,
            crate::vk::KHR_DISPLAY_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_display instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_display instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_display_properties_khr:
                PFN_vkGetPhysicalDeviceDisplayPropertiesKHR,
            pub get_physical_device_display_plane_properties_khr:
                PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR,
            pub get_display_plane_supported_displays_khr: PFN_vkGetDisplayPlaneSupportedDisplaysKHR,
            pub get_display_mode_properties_khr: PFN_vkGetDisplayModePropertiesKHR,
            pub create_display_mode_khr: PFN_vkCreateDisplayModeKHR,
            pub get_display_plane_capabilities_khr: PFN_vkGetDisplayPlaneCapabilitiesKHR,
            pub create_display_plane_surface_khr: PFN_vkCreateDisplayPlaneSurfaceKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_display_properties_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_display_properties_khr(
                            _physical_device: PhysicalDevice,
                            _p_property_count: *mut u32,
                            _p_properties: *mut DisplayPropertiesKHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_display_properties_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceDisplayPropertiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_display_properties_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_display_plane_properties_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_display_plane_properties_khr(
                            _physical_device: PhysicalDevice,
                            _p_property_count: *mut u32,
                            _p_properties: *mut DisplayPlanePropertiesKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_display_plane_properties_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceDisplayPlanePropertiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_display_plane_properties_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_display_plane_supported_displays_khr: unsafe {
                        unsafe extern "system" fn get_display_plane_supported_displays_khr(
                            _physical_device: PhysicalDevice,
                            _plane_index: u32,
                            _p_display_count: *mut u32,
                            _p_displays: *mut DisplayKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_display_plane_supported_displays_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDisplayPlaneSupportedDisplaysKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_display_plane_supported_displays_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_display_mode_properties_khr: unsafe {
                        unsafe extern "system" fn get_display_mode_properties_khr(
                            _physical_device: PhysicalDevice,
                            _display: DisplayKHR,
                            _p_property_count: *mut u32,
                            _p_properties: *mut DisplayModePropertiesKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_display_mode_properties_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetDisplayModePropertiesKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_display_mode_properties_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    create_display_mode_khr: unsafe {
                        unsafe extern "system" fn create_display_mode_khr(
                            _physical_device: PhysicalDevice,
                            _display: DisplayKHR,
                            _p_create_info: *const DisplayModeCreateInfoKHR<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_mode: *mut DisplayModeKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_display_mode_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateDisplayModeKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_display_mode_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_display_plane_capabilities_khr: unsafe {
                        unsafe extern "system" fn get_display_plane_capabilities_khr(
                            _physical_device: PhysicalDevice,
                            _mode: DisplayModeKHR,
                            _plane_index: u32,
                            _p_capabilities: *mut DisplayPlaneCapabilitiesKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_display_plane_capabilities_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDisplayPlaneCapabilitiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_display_plane_capabilities_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    create_display_plane_surface_khr: unsafe {
                        unsafe extern "system" fn create_display_plane_surface_khr(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const DisplaySurfaceCreateInfoKHR<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_surface: *mut SurfaceKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_display_plane_surface_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCreateDisplayPlaneSurfaceKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            create_display_plane_surface_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_display_swapchain"]
    pub mod display_swapchain {
        use super::super::*;
        pub use {
            crate::vk::KHR_DISPLAY_SWAPCHAIN_NAME as NAME,
            crate::vk::KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_display_swapchain device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_display_swapchain device-level function pointers"]
        pub struct DeviceFn {
            pub create_shared_swapchains_khr: PFN_vkCreateSharedSwapchainsKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_shared_swapchains_khr: unsafe {
                        unsafe extern "system" fn create_shared_swapchains_khr(
                            _device: crate::vk::Device,
                            _swapchain_count: u32,
                            _p_create_infos: *const SwapchainCreateInfoKHR<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_swapchains: *mut SwapchainKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_shared_swapchains_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateSharedSwapchainsKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_shared_swapchains_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_xlib_surface"]
    pub mod xlib_surface {
        use super::super::*;
        pub use {
            crate::vk::KHR_XLIB_SURFACE_NAME as NAME,
            crate::vk::KHR_XLIB_SURFACE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_xlib_surface instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_xlib_surface instance-level function pointers"]
        pub struct InstanceFn {
            pub create_xlib_surface_khr: PFN_vkCreateXlibSurfaceKHR,
            pub get_physical_device_xlib_presentation_support_khr:
                PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_xlib_surface_khr: unsafe {
                        unsafe extern "system" fn create_xlib_surface_khr(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const XlibSurfaceCreateInfoKHR<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_surface: *mut SurfaceKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_xlib_surface_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateXlibSurfaceKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_xlib_surface_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_xlib_presentation_support_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_xlib_presentation_support_khr(
                            _physical_device: PhysicalDevice,
                            _queue_family_index: u32,
                            _dpy: *mut Display,
                            _visual_id: VisualID,
                        ) -> Bool32 {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_xlib_presentation_support_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceXlibPresentationSupportKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_xlib_presentation_support_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_xcb_surface"]
    pub mod xcb_surface {
        use super::super::*;
        pub use {
            crate::vk::KHR_XCB_SURFACE_NAME as NAME,
            crate::vk::KHR_XCB_SURFACE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_xcb_surface instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_xcb_surface instance-level function pointers"]
        pub struct InstanceFn {
            pub create_xcb_surface_khr: PFN_vkCreateXcbSurfaceKHR,
            pub get_physical_device_xcb_presentation_support_khr:
                PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_xcb_surface_khr: unsafe {
                        unsafe extern "system" fn create_xcb_surface_khr(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const XcbSurfaceCreateInfoKHR<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_surface: *mut SurfaceKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_xcb_surface_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateXcbSurfaceKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_xcb_surface_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_xcb_presentation_support_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_xcb_presentation_support_khr(
                            _physical_device: PhysicalDevice,
                            _queue_family_index: u32,
                            _connection: *mut xcb_connection_t,
                            _visual_id: xcb_visualid_t,
                        ) -> Bool32 {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_xcb_presentation_support_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceXcbPresentationSupportKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_xcb_presentation_support_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_wayland_surface"]
    pub mod wayland_surface {
        use super::super::*;
        pub use {
            crate::vk::KHR_WAYLAND_SURFACE_NAME as NAME,
            crate::vk::KHR_WAYLAND_SURFACE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_wayland_surface instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_wayland_surface instance-level function pointers"]
        pub struct InstanceFn {
            pub create_wayland_surface_khr: PFN_vkCreateWaylandSurfaceKHR,
            pub get_physical_device_wayland_presentation_support_khr:
                PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_wayland_surface_khr: unsafe {
                        unsafe extern "system" fn create_wayland_surface_khr(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const WaylandSurfaceCreateInfoKHR<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_surface: *mut SurfaceKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_wayland_surface_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateWaylandSurfaceKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_wayland_surface_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_wayland_presentation_support_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_wayland_presentation_support_khr(
                            _physical_device: PhysicalDevice,
                            _queue_family_index: u32,
                            _display: *mut wl_display,
                        ) -> Bool32 {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_wayland_presentation_support_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceWaylandPresentationSupportKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_wayland_presentation_support_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_android_surface"]
    pub mod android_surface {
        use super::super::*;
        pub use {
            crate::vk::KHR_ANDROID_SURFACE_NAME as NAME,
            crate::vk::KHR_ANDROID_SURFACE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_android_surface instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_android_surface instance-level function pointers"]
        pub struct InstanceFn {
            pub create_android_surface_khr: PFN_vkCreateAndroidSurfaceKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_android_surface_khr: unsafe {
                        unsafe extern "system" fn create_android_surface_khr(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const AndroidSurfaceCreateInfoKHR<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_surface: *mut SurfaceKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_android_surface_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateAndroidSurfaceKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_android_surface_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_win32_surface"]
    pub mod win32_surface {
        use super::super::*;
        pub use {
            crate::vk::KHR_WIN32_SURFACE_NAME as NAME,
            crate::vk::KHR_WIN32_SURFACE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_win32_surface instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_win32_surface instance-level function pointers"]
        pub struct InstanceFn {
            pub create_win32_surface_khr: PFN_vkCreateWin32SurfaceKHR,
            pub get_physical_device_win32_presentation_support_khr:
                PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_win32_surface_khr: unsafe {
                        unsafe extern "system" fn create_win32_surface_khr(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const Win32SurfaceCreateInfoKHR<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_surface: *mut SurfaceKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_win32_surface_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateWin32SurfaceKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_win32_surface_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_win32_presentation_support_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_win32_presentation_support_khr(
                            _physical_device: PhysicalDevice,
                            _queue_family_index: u32,
                        ) -> Bool32 {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_win32_presentation_support_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceWin32PresentationSupportKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_win32_presentation_support_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_sampler_mirror_clamp_to_edge"]
    pub mod sampler_mirror_clamp_to_edge {
        use super::super::*;
        pub use {
            crate::vk::KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_NAME as NAME,
            crate::vk::KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_video_queue"]
    pub mod video_queue {
        use super::super::*;
        pub use {
            crate::vk::KHR_VIDEO_QUEUE_NAME as NAME,
            crate::vk::KHR_VIDEO_QUEUE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_video_queue instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_video_queue instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_video_capabilities_khr:
                PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR,
            pub get_physical_device_video_format_properties_khr:
                PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_video_capabilities_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_video_capabilities_khr(
                            _physical_device: PhysicalDevice,
                            _p_video_profile: *const VideoProfileInfoKHR<'_>,
                            _p_capabilities: *mut VideoCapabilitiesKHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_video_capabilities_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceVideoCapabilitiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_video_capabilities_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_video_format_properties_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_video_format_properties_khr(
                            _physical_device: PhysicalDevice,
                            _p_video_format_info: *const PhysicalDeviceVideoFormatInfoKHR<'_>,
                            _p_video_format_property_count: *mut u32,
                            _p_video_format_properties: *mut VideoFormatPropertiesKHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_video_format_properties_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceVideoFormatPropertiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_video_format_properties_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
        #[doc = "VK_KHR_video_queue device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_video_queue device-level function pointers"]
        pub struct DeviceFn {
            pub create_video_session_khr: PFN_vkCreateVideoSessionKHR,
            pub destroy_video_session_khr: PFN_vkDestroyVideoSessionKHR,
            pub get_video_session_memory_requirements_khr:
                PFN_vkGetVideoSessionMemoryRequirementsKHR,
            pub bind_video_session_memory_khr: PFN_vkBindVideoSessionMemoryKHR,
            pub create_video_session_parameters_khr: PFN_vkCreateVideoSessionParametersKHR,
            pub update_video_session_parameters_khr: PFN_vkUpdateVideoSessionParametersKHR,
            pub destroy_video_session_parameters_khr: PFN_vkDestroyVideoSessionParametersKHR,
            pub cmd_begin_video_coding_khr: PFN_vkCmdBeginVideoCodingKHR,
            pub cmd_end_video_coding_khr: PFN_vkCmdEndVideoCodingKHR,
            pub cmd_control_video_coding_khr: PFN_vkCmdControlVideoCodingKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_video_session_khr: unsafe {
                        unsafe extern "system" fn create_video_session_khr(
                            _device: crate::vk::Device,
                            _p_create_info: *const VideoSessionCreateInfoKHR<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_video_session: *mut VideoSessionKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_video_session_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateVideoSessionKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_video_session_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_video_session_khr: unsafe {
                        unsafe extern "system" fn destroy_video_session_khr(
                            _device: crate::vk::Device,
                            _video_session: VideoSessionKHR,
                            _p_allocator: *const AllocationCallbacks<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(destroy_video_session_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkDestroyVideoSessionKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_video_session_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_video_session_memory_requirements_khr: unsafe {
                        unsafe extern "system" fn get_video_session_memory_requirements_khr(
                            _device: crate::vk::Device,
                            _video_session: VideoSessionKHR,
                            _p_memory_requirements_count: *mut u32,
                            _p_memory_requirements: *mut VideoSessionMemoryRequirementsKHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_video_session_memory_requirements_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetVideoSessionMemoryRequirementsKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_video_session_memory_requirements_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    bind_video_session_memory_khr: unsafe {
                        unsafe extern "system" fn bind_video_session_memory_khr(
                            _device: crate::vk::Device,
                            _video_session: VideoSessionKHR,
                            _bind_session_memory_info_count: u32,
                            _p_bind_session_memory_infos: *const BindVideoSessionMemoryInfoKHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(bind_video_session_memory_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkBindVideoSessionMemoryKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            bind_video_session_memory_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    create_video_session_parameters_khr: unsafe {
                        unsafe extern "system" fn create_video_session_parameters_khr(
                            _device: crate::vk::Device,
                            _p_create_info: *const VideoSessionParametersCreateInfoKHR<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_video_session_parameters: *mut VideoSessionParametersKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_video_session_parameters_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCreateVideoSessionParametersKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            create_video_session_parameters_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    update_video_session_parameters_khr: unsafe {
                        unsafe extern "system" fn update_video_session_parameters_khr(
                            _device: crate::vk::Device,
                            _video_session_parameters: VideoSessionParametersKHR,
                            _p_update_info: *const VideoSessionParametersUpdateInfoKHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(update_video_session_parameters_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkUpdateVideoSessionParametersKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            update_video_session_parameters_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_video_session_parameters_khr: unsafe {
                        unsafe extern "system" fn destroy_video_session_parameters_khr(
                            _device: crate::vk::Device,
                            _video_session_parameters: VideoSessionParametersKHR,
                            _p_allocator: *const AllocationCallbacks<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(destroy_video_session_parameters_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkDestroyVideoSessionParametersKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_video_session_parameters_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_begin_video_coding_khr: unsafe {
                        unsafe extern "system" fn cmd_begin_video_coding_khr(
                            _command_buffer: CommandBuffer,
                            _p_begin_info: *const VideoBeginCodingInfoKHR<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_begin_video_coding_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginVideoCodingKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_begin_video_coding_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_end_video_coding_khr: unsafe {
                        unsafe extern "system" fn cmd_end_video_coding_khr(
                            _command_buffer: CommandBuffer,
                            _p_end_coding_info: *const VideoEndCodingInfoKHR<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_end_video_coding_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdEndVideoCodingKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_end_video_coding_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_control_video_coding_khr: unsafe {
                        unsafe extern "system" fn cmd_control_video_coding_khr(
                            _command_buffer: CommandBuffer,
                            _p_coding_control_info: *const VideoCodingControlInfoKHR<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_control_video_coding_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdControlVideoCodingKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_control_video_coding_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_video_decode_queue"]
    pub mod video_decode_queue {
        use super::super::*;
        pub use {
            crate::vk::KHR_VIDEO_DECODE_QUEUE_NAME as NAME,
            crate::vk::KHR_VIDEO_DECODE_QUEUE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_video_decode_queue device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_video_decode_queue device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_decode_video_khr: PFN_vkCmdDecodeVideoKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_decode_video_khr: unsafe {
                        unsafe extern "system" fn cmd_decode_video_khr(
                            _command_buffer: CommandBuffer,
                            _p_decode_info: *const VideoDecodeInfoKHR<'_>,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_decode_video_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdDecodeVideoKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_decode_video_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_video_encode_h264"]
    pub mod video_encode_h264 {
        use super::super::*;
        pub use {
            crate::vk::KHR_VIDEO_ENCODE_H264_NAME as NAME,
            crate::vk::KHR_VIDEO_ENCODE_H264_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_video_encode_h265"]
    pub mod video_encode_h265 {
        use super::super::*;
        pub use {
            crate::vk::KHR_VIDEO_ENCODE_H265_NAME as NAME,
            crate::vk::KHR_VIDEO_ENCODE_H265_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_video_decode_h264"]
    pub mod video_decode_h264 {
        use super::super::*;
        pub use {
            crate::vk::KHR_VIDEO_DECODE_H264_NAME as NAME,
            crate::vk::KHR_VIDEO_DECODE_H264_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_dynamic_rendering"]
    pub mod dynamic_rendering {
        use super::super::*;
        pub use {
            crate::vk::KHR_DYNAMIC_RENDERING_NAME as NAME,
            crate::vk::KHR_DYNAMIC_RENDERING_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_dynamic_rendering device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_dynamic_rendering device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_begin_rendering_khr: PFN_vkCmdBeginRendering,
            pub cmd_end_rendering_khr: PFN_vkCmdEndRendering,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_begin_rendering_khr: unsafe {
                        unsafe extern "system" fn cmd_begin_rendering_khr(
                            _command_buffer: CommandBuffer,
                            _p_rendering_info: *const RenderingInfo<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_begin_rendering_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginRenderingKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_begin_rendering_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_end_rendering_khr: unsafe {
                        unsafe extern "system" fn cmd_end_rendering_khr(
                            _command_buffer: CommandBuffer,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_end_rendering_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdEndRenderingKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_end_rendering_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_multiview"]
    pub mod multiview {
        use super::super::*;
        pub use {
            crate::vk::KHR_MULTIVIEW_NAME as NAME,
            crate::vk::KHR_MULTIVIEW_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_get_physical_device_properties2"]
    pub mod get_physical_device_properties2 {
        use super::super::*;
        pub use {
            crate::vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_NAME as NAME,
            crate::vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_get_physical_device_properties2 instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_get_physical_device_properties2 instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_features2_khr: PFN_vkGetPhysicalDeviceFeatures2,
            pub get_physical_device_properties2_khr: PFN_vkGetPhysicalDeviceProperties2,
            pub get_physical_device_format_properties2_khr:
                PFN_vkGetPhysicalDeviceFormatProperties2,
            pub get_physical_device_image_format_properties2_khr:
                PFN_vkGetPhysicalDeviceImageFormatProperties2,
            pub get_physical_device_queue_family_properties2_khr:
                PFN_vkGetPhysicalDeviceQueueFamilyProperties2,
            pub get_physical_device_memory_properties2_khr:
                PFN_vkGetPhysicalDeviceMemoryProperties2,
            pub get_physical_device_sparse_image_format_properties2_khr:
                PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_features2_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_features2_khr(
                            _physical_device: PhysicalDevice,
                            _p_features: *mut PhysicalDeviceFeatures2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_features2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceFeatures2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_features2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_properties2_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_properties2_khr(
                            _physical_device: PhysicalDevice,
                            _p_properties: *mut PhysicalDeviceProperties2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_properties2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceProperties2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_properties2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_format_properties2_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_format_properties2_khr(
                            _physical_device: PhysicalDevice,
                            _format: Format,
                            _p_format_properties: *mut FormatProperties2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_format_properties2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceFormatProperties2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_format_properties2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_image_format_properties2_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_image_format_properties2_khr(
                            _physical_device: PhysicalDevice,
                            _p_image_format_info: *const PhysicalDeviceImageFormatInfo2<'_>,
                            _p_image_format_properties: *mut ImageFormatProperties2<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_image_format_properties2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceImageFormatProperties2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_image_format_properties2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_queue_family_properties2_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_queue_family_properties2_khr(
                            _physical_device: PhysicalDevice,
                            _p_queue_family_property_count: *mut u32,
                            _p_queue_family_properties: *mut QueueFamilyProperties2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_queue_family_properties2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceQueueFamilyProperties2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_queue_family_properties2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_memory_properties2_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_memory_properties2_khr(
                            _physical_device: PhysicalDevice,
                            _p_memory_properties: *mut PhysicalDeviceMemoryProperties2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_memory_properties2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceMemoryProperties2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_memory_properties2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_sparse_image_format_properties2_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_sparse_image_format_properties2_khr(
                            _physical_device: PhysicalDevice,
                            _p_format_info: *const PhysicalDeviceSparseImageFormatInfo2<'_>,
                            _p_property_count: *mut u32,
                            _p_properties: *mut SparseImageFormatProperties2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_sparse_image_format_properties2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceSparseImageFormatProperties2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_sparse_image_format_properties2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_device_group"]
    pub mod device_group {
        use super::super::*;
        pub use {
            crate::vk::KHR_DEVICE_GROUP_NAME as NAME,
            crate::vk::KHR_DEVICE_GROUP_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_device_group instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_device_group instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_present_rectangles_khr:
                PFN_vkGetPhysicalDevicePresentRectanglesKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_present_rectangles_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_present_rectangles_khr(
                            _physical_device: PhysicalDevice,
                            _surface: SurfaceKHR,
                            _p_rect_count: *mut u32,
                            _p_rects: *mut Rect2D,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_present_rectangles_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDevicePresentRectanglesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_present_rectangles_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
        #[doc = "VK_KHR_device_group device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_device_group device-level function pointers"]
        pub struct DeviceFn {
            pub get_device_group_peer_memory_features_khr: PFN_vkGetDeviceGroupPeerMemoryFeatures,
            pub cmd_set_device_mask_khr: PFN_vkCmdSetDeviceMask,
            pub cmd_dispatch_base_khr: PFN_vkCmdDispatchBase,
            pub get_device_group_present_capabilities_khr:
                PFN_vkGetDeviceGroupPresentCapabilitiesKHR,
            pub get_device_group_surface_present_modes_khr:
                PFN_vkGetDeviceGroupSurfacePresentModesKHR,
            pub acquire_next_image2_khr: PFN_vkAcquireNextImage2KHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_device_group_peer_memory_features_khr: unsafe {
                        unsafe extern "system" fn get_device_group_peer_memory_features_khr(
                            _device: crate::vk::Device,
                            _heap_index: u32,
                            _local_device_index: u32,
                            _remote_device_index: u32,
                            _p_peer_memory_features: *mut PeerMemoryFeatureFlags,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_device_group_peer_memory_features_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeviceGroupPeerMemoryFeaturesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_device_group_peer_memory_features_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_device_mask_khr: unsafe {
                        unsafe extern "system" fn cmd_set_device_mask_khr(
                            _command_buffer: CommandBuffer,
                            _device_mask: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_device_mask_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDeviceMaskKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_device_mask_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_dispatch_base_khr: unsafe {
                        unsafe extern "system" fn cmd_dispatch_base_khr(
                            _command_buffer: CommandBuffer,
                            _base_group_x: u32,
                            _base_group_y: u32,
                            _base_group_z: u32,
                            _group_count_x: u32,
                            _group_count_y: u32,
                            _group_count_z: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_dispatch_base_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdDispatchBaseKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_dispatch_base_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_device_group_present_capabilities_khr: unsafe {
                        unsafe extern "system" fn get_device_group_present_capabilities_khr(
                            _device: crate::vk::Device,
                            _p_device_group_present_capabilities : * mut DeviceGroupPresentCapabilitiesKHR < '_ >,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_device_group_present_capabilities_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeviceGroupPresentCapabilitiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_device_group_present_capabilities_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_device_group_surface_present_modes_khr: unsafe {
                        unsafe extern "system" fn get_device_group_surface_present_modes_khr(
                            _device: crate::vk::Device,
                            _surface: SurfaceKHR,
                            _p_modes: *mut DeviceGroupPresentModeFlagsKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_device_group_surface_present_modes_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeviceGroupSurfacePresentModesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_device_group_surface_present_modes_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    acquire_next_image2_khr: unsafe {
                        unsafe extern "system" fn acquire_next_image2_khr(
                            _device: crate::vk::Device,
                            _p_acquire_info: *const AcquireNextImageInfoKHR<'_>,
                            _p_image_index: *mut u32,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(acquire_next_image2_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkAcquireNextImage2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            acquire_next_image2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_shader_draw_parameters"]
    pub mod shader_draw_parameters {
        use super::super::*;
        pub use {
            crate::vk::KHR_SHADER_DRAW_PARAMETERS_NAME as NAME,
            crate::vk::KHR_SHADER_DRAW_PARAMETERS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_maintenance1"]
    pub mod maintenance1 {
        use super::super::*;
        pub use {
            crate::vk::KHR_MAINTENANCE1_NAME as NAME,
            crate::vk::KHR_MAINTENANCE1_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_maintenance1 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_maintenance1 device-level function pointers"]
        pub struct DeviceFn {
            pub trim_command_pool_khr: PFN_vkTrimCommandPool,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    trim_command_pool_khr: unsafe {
                        unsafe extern "system" fn trim_command_pool_khr(
                            _device: crate::vk::Device,
                            _command_pool: CommandPool,
                            _flags: CommandPoolTrimFlags,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(trim_command_pool_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkTrimCommandPoolKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            trim_command_pool_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_device_group_creation"]
    pub mod device_group_creation {
        use super::super::*;
        pub use {
            crate::vk::KHR_DEVICE_GROUP_CREATION_NAME as NAME,
            crate::vk::KHR_DEVICE_GROUP_CREATION_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_device_group_creation instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_device_group_creation instance-level function pointers"]
        pub struct InstanceFn {
            pub enumerate_physical_device_groups_khr: PFN_vkEnumeratePhysicalDeviceGroups,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    enumerate_physical_device_groups_khr: unsafe {
                        unsafe extern "system" fn enumerate_physical_device_groups_khr(
                            _instance: crate::vk::Instance,
                            _p_physical_device_group_count: *mut u32,
                            _p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties<
                                '_,
                            >,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(enumerate_physical_device_groups_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkEnumeratePhysicalDeviceGroupsKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            enumerate_physical_device_groups_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_external_memory_capabilities"]
    pub mod external_memory_capabilities {
        use super::super::*;
        pub use {
            crate::vk::KHR_EXTERNAL_MEMORY_CAPABILITIES_NAME as NAME,
            crate::vk::KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_external_memory_capabilities instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_external_memory_capabilities instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_external_buffer_properties_khr:
                PFN_vkGetPhysicalDeviceExternalBufferProperties,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_external_buffer_properties_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_external_buffer_properties_khr(
                            _physical_device: PhysicalDevice,
                            _p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo<'_>,
                            _p_external_buffer_properties: *mut ExternalBufferProperties<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_external_buffer_properties_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceExternalBufferPropertiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_external_buffer_properties_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_external_memory"]
    pub mod external_memory {
        use super::super::*;
        pub use {
            crate::vk::KHR_EXTERNAL_MEMORY_NAME as NAME,
            crate::vk::KHR_EXTERNAL_MEMORY_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_external_memory_win32"]
    pub mod external_memory_win32 {
        use super::super::*;
        pub use {
            crate::vk::KHR_EXTERNAL_MEMORY_WIN32_NAME as NAME,
            crate::vk::KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_external_memory_win32 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_external_memory_win32 device-level function pointers"]
        pub struct DeviceFn {
            pub get_memory_win32_handle_khr: PFN_vkGetMemoryWin32HandleKHR,
            pub get_memory_win32_handle_properties_khr: PFN_vkGetMemoryWin32HandlePropertiesKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_memory_win32_handle_khr: unsafe {
                        unsafe extern "system" fn get_memory_win32_handle_khr(
                            _device: crate::vk::Device,
                            _p_get_win32_handle_info: *const MemoryGetWin32HandleInfoKHR<'_>,
                            _p_handle: *mut HANDLE,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_memory_win32_handle_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetMemoryWin32HandleKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_memory_win32_handle_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_memory_win32_handle_properties_khr: unsafe {
                        unsafe extern "system" fn get_memory_win32_handle_properties_khr(
                            _device: crate::vk::Device,
                            _handle_type: ExternalMemoryHandleTypeFlags,
                            _handle: HANDLE,
                            _p_memory_win32_handle_properties: *mut MemoryWin32HandlePropertiesKHR<
                                '_,
                            >,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_memory_win32_handle_properties_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetMemoryWin32HandlePropertiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_memory_win32_handle_properties_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_external_memory_fd"]
    pub mod external_memory_fd {
        use super::super::*;
        pub use {
            crate::vk::KHR_EXTERNAL_MEMORY_FD_NAME as NAME,
            crate::vk::KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_external_memory_fd device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_external_memory_fd device-level function pointers"]
        pub struct DeviceFn {
            pub get_memory_fd_khr: PFN_vkGetMemoryFdKHR,
            pub get_memory_fd_properties_khr: PFN_vkGetMemoryFdPropertiesKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_memory_fd_khr: unsafe {
                        unsafe extern "system" fn get_memory_fd_khr(
                            _device: crate::vk::Device,
                            _p_get_fd_info: *const MemoryGetFdInfoKHR<'_>,
                            _p_fd: *mut c_int,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(get_memory_fd_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetMemoryFdKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_memory_fd_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_memory_fd_properties_khr: unsafe {
                        unsafe extern "system" fn get_memory_fd_properties_khr(
                            _device: crate::vk::Device,
                            _handle_type: ExternalMemoryHandleTypeFlags,
                            _fd: c_int,
                            _p_memory_fd_properties: *mut MemoryFdPropertiesKHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_memory_fd_properties_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetMemoryFdPropertiesKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_memory_fd_properties_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_win32_keyed_mutex"]
    pub mod win32_keyed_mutex {
        use super::super::*;
        pub use {
            crate::vk::KHR_WIN32_KEYED_MUTEX_NAME as NAME,
            crate::vk::KHR_WIN32_KEYED_MUTEX_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_external_semaphore_capabilities"]
    pub mod external_semaphore_capabilities {
        use super::super::*;
        pub use {
            crate::vk::KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_NAME as NAME,
            crate::vk::KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_external_semaphore_capabilities instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_external_semaphore_capabilities instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_external_semaphore_properties_khr:
                PFN_vkGetPhysicalDeviceExternalSemaphoreProperties,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_external_semaphore_properties_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_external_semaphore_properties_khr(
                            _physical_device: PhysicalDevice,
                            _p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo<
                                '_,
                            >,
                            _p_external_semaphore_properties: *mut ExternalSemaphoreProperties<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_external_semaphore_properties_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceExternalSemaphorePropertiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_external_semaphore_properties_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_external_semaphore"]
    pub mod external_semaphore {
        use super::super::*;
        pub use {
            crate::vk::KHR_EXTERNAL_SEMAPHORE_NAME as NAME,
            crate::vk::KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_external_semaphore_win32"]
    pub mod external_semaphore_win32 {
        use super::super::*;
        pub use {
            crate::vk::KHR_EXTERNAL_SEMAPHORE_WIN32_NAME as NAME,
            crate::vk::KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_external_semaphore_win32 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_external_semaphore_win32 device-level function pointers"]
        pub struct DeviceFn {
            pub import_semaphore_win32_handle_khr: PFN_vkImportSemaphoreWin32HandleKHR,
            pub get_semaphore_win32_handle_khr: PFN_vkGetSemaphoreWin32HandleKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    import_semaphore_win32_handle_khr: unsafe {
                        unsafe extern "system" fn import_semaphore_win32_handle_khr(
                            _device: crate::vk::Device,
                            _p_import_semaphore_win32_handle_info : * const ImportSemaphoreWin32HandleInfoKHR < '_ >,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(import_semaphore_win32_handle_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkImportSemaphoreWin32HandleKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            import_semaphore_win32_handle_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_semaphore_win32_handle_khr: unsafe {
                        unsafe extern "system" fn get_semaphore_win32_handle_khr(
                            _device: crate::vk::Device,
                            _p_get_win32_handle_info: *const SemaphoreGetWin32HandleInfoKHR<'_>,
                            _p_handle: *mut HANDLE,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_semaphore_win32_handle_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetSemaphoreWin32HandleKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_semaphore_win32_handle_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_external_semaphore_fd"]
    pub mod external_semaphore_fd {
        use super::super::*;
        pub use {
            crate::vk::KHR_EXTERNAL_SEMAPHORE_FD_NAME as NAME,
            crate::vk::KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_external_semaphore_fd device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_external_semaphore_fd device-level function pointers"]
        pub struct DeviceFn {
            pub import_semaphore_fd_khr: PFN_vkImportSemaphoreFdKHR,
            pub get_semaphore_fd_khr: PFN_vkGetSemaphoreFdKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    import_semaphore_fd_khr: unsafe {
                        unsafe extern "system" fn import_semaphore_fd_khr(
                            _device: crate::vk::Device,
                            _p_import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(import_semaphore_fd_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkImportSemaphoreFdKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            import_semaphore_fd_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_semaphore_fd_khr: unsafe {
                        unsafe extern "system" fn get_semaphore_fd_khr(
                            _device: crate::vk::Device,
                            _p_get_fd_info: *const SemaphoreGetFdInfoKHR<'_>,
                            _p_fd: *mut c_int,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(get_semaphore_fd_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetSemaphoreFdKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_semaphore_fd_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_push_descriptor"]
    pub mod push_descriptor {
        use super::super::*;
        pub use {
            crate::vk::KHR_PUSH_DESCRIPTOR_NAME as NAME,
            crate::vk::KHR_PUSH_DESCRIPTOR_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_push_descriptor device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_push_descriptor device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_push_descriptor_set_khr: PFN_vkCmdPushDescriptorSetKHR,
            pub cmd_push_descriptor_set_with_template_khr:
                PFN_vkCmdPushDescriptorSetWithTemplateKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_push_descriptor_set_khr: unsafe {
                        unsafe extern "system" fn cmd_push_descriptor_set_khr(
                            _command_buffer: CommandBuffer,
                            _pipeline_bind_point: PipelineBindPoint,
                            _layout: PipelineLayout,
                            _set: u32,
                            _descriptor_write_count: u32,
                            _p_descriptor_writes: *const WriteDescriptorSet<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_push_descriptor_set_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdPushDescriptorSetKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_push_descriptor_set_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_push_descriptor_set_with_template_khr: unsafe {
                        unsafe extern "system" fn cmd_push_descriptor_set_with_template_khr(
                            _command_buffer: CommandBuffer,
                            _descriptor_update_template: DescriptorUpdateTemplate,
                            _layout: PipelineLayout,
                            _set: u32,
                            _p_data: *const c_void,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_push_descriptor_set_with_template_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdPushDescriptorSetWithTemplateKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_push_descriptor_set_with_template_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_shader_float16_int8"]
    pub mod shader_float16_int8 {
        use super::super::*;
        pub use {
            crate::vk::KHR_SHADER_FLOAT16_INT8_NAME as NAME,
            crate::vk::KHR_SHADER_FLOAT16_INT8_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_16bit_storage"]
    pub mod _16bit_storage {
        use super::super::*;
        pub use {
            crate::vk::KHR_16BIT_STORAGE_NAME as NAME,
            crate::vk::KHR_16BIT_STORAGE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_incremental_present"]
    pub mod incremental_present {
        use super::super::*;
        pub use {
            crate::vk::KHR_INCREMENTAL_PRESENT_NAME as NAME,
            crate::vk::KHR_INCREMENTAL_PRESENT_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_descriptor_update_template"]
    pub mod descriptor_update_template {
        use super::super::*;
        pub use {
            crate::vk::KHR_DESCRIPTOR_UPDATE_TEMPLATE_NAME as NAME,
            crate::vk::KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_descriptor_update_template device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_descriptor_update_template device-level function pointers"]
        pub struct DeviceFn {
            pub create_descriptor_update_template_khr: PFN_vkCreateDescriptorUpdateTemplate,
            pub destroy_descriptor_update_template_khr: PFN_vkDestroyDescriptorUpdateTemplate,
            pub update_descriptor_set_with_template_khr: PFN_vkUpdateDescriptorSetWithTemplate,
            pub cmd_push_descriptor_set_with_template_khr:
                PFN_vkCmdPushDescriptorSetWithTemplateKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_descriptor_update_template_khr: unsafe {
                        unsafe extern "system" fn create_descriptor_update_template_khr(
                            _device: crate::vk::Device,
                            _p_create_info: *const DescriptorUpdateTemplateCreateInfo<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_descriptor_update_template: *mut DescriptorUpdateTemplate,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_descriptor_update_template_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCreateDescriptorUpdateTemplateKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            create_descriptor_update_template_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_descriptor_update_template_khr: unsafe {
                        unsafe extern "system" fn destroy_descriptor_update_template_khr(
                            _device: crate::vk::Device,
                            _descriptor_update_template: DescriptorUpdateTemplate,
                            _p_allocator: *const AllocationCallbacks<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(destroy_descriptor_update_template_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkDestroyDescriptorUpdateTemplateKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_descriptor_update_template_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    update_descriptor_set_with_template_khr: unsafe {
                        unsafe extern "system" fn update_descriptor_set_with_template_khr(
                            _device: crate::vk::Device,
                            _descriptor_set: DescriptorSet,
                            _descriptor_update_template: DescriptorUpdateTemplate,
                            _p_data: *const c_void,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(update_descriptor_set_with_template_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkUpdateDescriptorSetWithTemplateKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            update_descriptor_set_with_template_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_push_descriptor_set_with_template_khr: unsafe {
                        unsafe extern "system" fn cmd_push_descriptor_set_with_template_khr(
                            _command_buffer: CommandBuffer,
                            _descriptor_update_template: DescriptorUpdateTemplate,
                            _layout: PipelineLayout,
                            _set: u32,
                            _p_data: *const c_void,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_push_descriptor_set_with_template_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdPushDescriptorSetWithTemplateKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_push_descriptor_set_with_template_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_imageless_framebuffer"]
    pub mod imageless_framebuffer {
        use super::super::*;
        pub use {
            crate::vk::KHR_IMAGELESS_FRAMEBUFFER_NAME as NAME,
            crate::vk::KHR_IMAGELESS_FRAMEBUFFER_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_create_renderpass2"]
    pub mod create_renderpass2 {
        use super::super::*;
        pub use {
            crate::vk::KHR_CREATE_RENDERPASS2_NAME as NAME,
            crate::vk::KHR_CREATE_RENDERPASS2_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_create_renderpass2 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_create_renderpass2 device-level function pointers"]
        pub struct DeviceFn {
            pub create_render_pass2_khr: PFN_vkCreateRenderPass2,
            pub cmd_begin_render_pass2_khr: PFN_vkCmdBeginRenderPass2,
            pub cmd_next_subpass2_khr: PFN_vkCmdNextSubpass2,
            pub cmd_end_render_pass2_khr: PFN_vkCmdEndRenderPass2,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_render_pass2_khr: unsafe {
                        unsafe extern "system" fn create_render_pass2_khr(
                            _device: crate::vk::Device,
                            _p_create_info: *const RenderPassCreateInfo2<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_render_pass: *mut RenderPass,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_render_pass2_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateRenderPass2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_render_pass2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_begin_render_pass2_khr: unsafe {
                        unsafe extern "system" fn cmd_begin_render_pass2_khr(
                            _command_buffer: CommandBuffer,
                            _p_render_pass_begin: *const RenderPassBeginInfo<'_>,
                            _p_subpass_begin_info: *const SubpassBeginInfo<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_begin_render_pass2_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginRenderPass2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_begin_render_pass2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_next_subpass2_khr: unsafe {
                        unsafe extern "system" fn cmd_next_subpass2_khr(
                            _command_buffer: CommandBuffer,
                            _p_subpass_begin_info: *const SubpassBeginInfo<'_>,
                            _p_subpass_end_info: *const SubpassEndInfo<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_next_subpass2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdNextSubpass2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_next_subpass2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_end_render_pass2_khr: unsafe {
                        unsafe extern "system" fn cmd_end_render_pass2_khr(
                            _command_buffer: CommandBuffer,
                            _p_subpass_end_info: *const SubpassEndInfo<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_end_render_pass2_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdEndRenderPass2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_end_render_pass2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_shared_presentable_image"]
    pub mod shared_presentable_image {
        use super::super::*;
        pub use {
            crate::vk::KHR_SHARED_PRESENTABLE_IMAGE_NAME as NAME,
            crate::vk::KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_shared_presentable_image device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_shared_presentable_image device-level function pointers"]
        pub struct DeviceFn {
            pub get_swapchain_status_khr: PFN_vkGetSwapchainStatusKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_swapchain_status_khr: unsafe {
                        unsafe extern "system" fn get_swapchain_status_khr(
                            _device: crate::vk::Device,
                            _swapchain: SwapchainKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_swapchain_status_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetSwapchainStatusKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_swapchain_status_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_external_fence_capabilities"]
    pub mod external_fence_capabilities {
        use super::super::*;
        pub use {
            crate::vk::KHR_EXTERNAL_FENCE_CAPABILITIES_NAME as NAME,
            crate::vk::KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_external_fence_capabilities instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_external_fence_capabilities instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_external_fence_properties_khr:
                PFN_vkGetPhysicalDeviceExternalFenceProperties,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_external_fence_properties_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_external_fence_properties_khr(
                            _physical_device: PhysicalDevice,
                            _p_external_fence_info: *const PhysicalDeviceExternalFenceInfo<'_>,
                            _p_external_fence_properties: *mut ExternalFenceProperties<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_external_fence_properties_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceExternalFencePropertiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_external_fence_properties_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_external_fence"]
    pub mod external_fence {
        use super::super::*;
        pub use {
            crate::vk::KHR_EXTERNAL_FENCE_NAME as NAME,
            crate::vk::KHR_EXTERNAL_FENCE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_external_fence_win32"]
    pub mod external_fence_win32 {
        use super::super::*;
        pub use {
            crate::vk::KHR_EXTERNAL_FENCE_WIN32_NAME as NAME,
            crate::vk::KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_external_fence_win32 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_external_fence_win32 device-level function pointers"]
        pub struct DeviceFn {
            pub import_fence_win32_handle_khr: PFN_vkImportFenceWin32HandleKHR,
            pub get_fence_win32_handle_khr: PFN_vkGetFenceWin32HandleKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    import_fence_win32_handle_khr: unsafe {
                        unsafe extern "system" fn import_fence_win32_handle_khr(
                            _device: crate::vk::Device,
                            _p_import_fence_win32_handle_info: *const ImportFenceWin32HandleInfoKHR<
                                '_,
                            >,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(import_fence_win32_handle_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkImportFenceWin32HandleKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            import_fence_win32_handle_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_fence_win32_handle_khr: unsafe {
                        unsafe extern "system" fn get_fence_win32_handle_khr(
                            _device: crate::vk::Device,
                            _p_get_win32_handle_info: *const FenceGetWin32HandleInfoKHR<'_>,
                            _p_handle: *mut HANDLE,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_fence_win32_handle_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetFenceWin32HandleKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_fence_win32_handle_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_external_fence_fd"]
    pub mod external_fence_fd {
        use super::super::*;
        pub use {
            crate::vk::KHR_EXTERNAL_FENCE_FD_NAME as NAME,
            crate::vk::KHR_EXTERNAL_FENCE_FD_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_external_fence_fd device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_external_fence_fd device-level function pointers"]
        pub struct DeviceFn {
            pub import_fence_fd_khr: PFN_vkImportFenceFdKHR,
            pub get_fence_fd_khr: PFN_vkGetFenceFdKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    import_fence_fd_khr: unsafe {
                        unsafe extern "system" fn import_fence_fd_khr(
                            _device: crate::vk::Device,
                            _p_import_fence_fd_info: *const ImportFenceFdInfoKHR<'_>,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(import_fence_fd_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkImportFenceFdKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            import_fence_fd_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_fence_fd_khr: unsafe {
                        unsafe extern "system" fn get_fence_fd_khr(
                            _device: crate::vk::Device,
                            _p_get_fd_info: *const FenceGetFdInfoKHR<'_>,
                            _p_fd: *mut c_int,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(get_fence_fd_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetFenceFdKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_fence_fd_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_performance_query"]
    pub mod performance_query {
        use super::super::*;
        pub use {
            crate::vk::KHR_PERFORMANCE_QUERY_NAME as NAME,
            crate::vk::KHR_PERFORMANCE_QUERY_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_performance_query instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_performance_query instance-level function pointers"]
        pub struct InstanceFn {
            pub enumerate_physical_device_queue_family_performance_query_counters_khr:
                PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR,
            pub get_physical_device_queue_family_performance_query_passes_khr:
                PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    enumerate_physical_device_queue_family_performance_query_counters_khr: unsafe {
                        unsafe extern "system" fn enumerate_physical_device_queue_family_performance_query_counters_khr(
                            _physical_device: PhysicalDevice,
                            _queue_family_index: u32,
                            _p_counter_count: *mut u32,
                            _p_counters: *mut PerformanceCounterKHR<'_>,
                            _p_counter_descriptions: *mut PerformanceCounterDescriptionKHR<'_>,
                        ) -> Result {
                            panic ! (concat ! ("Unable to load " , stringify ! (enumerate_physical_device_queue_family_performance_query_counters_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            enumerate_physical_device_queue_family_performance_query_counters_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_queue_family_performance_query_passes_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_queue_family_performance_query_passes_khr(
                            _physical_device: PhysicalDevice,
                            _p_performance_query_create_info : * const QueryPoolPerformanceCreateInfoKHR < '_ >,
                            _p_num_passes: *mut u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(
                                    get_physical_device_queue_family_performance_query_passes_khr
                                )
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_queue_family_performance_query_passes_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
        #[doc = "VK_KHR_performance_query device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_performance_query device-level function pointers"]
        pub struct DeviceFn {
            pub acquire_profiling_lock_khr: PFN_vkAcquireProfilingLockKHR,
            pub release_profiling_lock_khr: PFN_vkReleaseProfilingLockKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    acquire_profiling_lock_khr: unsafe {
                        unsafe extern "system" fn acquire_profiling_lock_khr(
                            _device: crate::vk::Device,
                            _p_info: *const AcquireProfilingLockInfoKHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(acquire_profiling_lock_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkAcquireProfilingLockKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            acquire_profiling_lock_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    release_profiling_lock_khr: unsafe {
                        unsafe extern "system" fn release_profiling_lock_khr(
                            _device: crate::vk::Device,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(release_profiling_lock_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkReleaseProfilingLockKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            release_profiling_lock_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_maintenance2"]
    pub mod maintenance2 {
        use super::super::*;
        pub use {
            crate::vk::KHR_MAINTENANCE2_NAME as NAME,
            crate::vk::KHR_MAINTENANCE2_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_get_surface_capabilities2"]
    pub mod get_surface_capabilities2 {
        use super::super::*;
        pub use {
            crate::vk::KHR_GET_SURFACE_CAPABILITIES2_NAME as NAME,
            crate::vk::KHR_GET_SURFACE_CAPABILITIES2_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_get_surface_capabilities2 instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_get_surface_capabilities2 instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_surface_capabilities2_khr:
                PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR,
            pub get_physical_device_surface_formats2_khr: PFN_vkGetPhysicalDeviceSurfaceFormats2KHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_surface_capabilities2_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_surface_capabilities2_khr(
                            _physical_device: PhysicalDevice,
                            _p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
                            _p_surface_capabilities: *mut SurfaceCapabilities2KHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_surface_capabilities2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceSurfaceCapabilities2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_surface_capabilities2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_surface_formats2_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_surface_formats2_khr(
                            _physical_device: PhysicalDevice,
                            _p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
                            _p_surface_format_count: *mut u32,
                            _p_surface_formats: *mut SurfaceFormat2KHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_surface_formats2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceSurfaceFormats2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_surface_formats2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_variable_pointers"]
    pub mod variable_pointers {
        use super::super::*;
        pub use {
            crate::vk::KHR_VARIABLE_POINTERS_NAME as NAME,
            crate::vk::KHR_VARIABLE_POINTERS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_get_display_properties2"]
    pub mod get_display_properties2 {
        use super::super::*;
        pub use {
            crate::vk::KHR_GET_DISPLAY_PROPERTIES2_NAME as NAME,
            crate::vk::KHR_GET_DISPLAY_PROPERTIES2_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_get_display_properties2 instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_get_display_properties2 instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_display_properties2_khr:
                PFN_vkGetPhysicalDeviceDisplayProperties2KHR,
            pub get_physical_device_display_plane_properties2_khr:
                PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR,
            pub get_display_mode_properties2_khr: PFN_vkGetDisplayModeProperties2KHR,
            pub get_display_plane_capabilities2_khr: PFN_vkGetDisplayPlaneCapabilities2KHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_display_properties2_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_display_properties2_khr(
                            _physical_device: PhysicalDevice,
                            _p_property_count: *mut u32,
                            _p_properties: *mut DisplayProperties2KHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_display_properties2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceDisplayProperties2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_display_properties2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_display_plane_properties2_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_display_plane_properties2_khr(
                            _physical_device: PhysicalDevice,
                            _p_property_count: *mut u32,
                            _p_properties: *mut DisplayPlaneProperties2KHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_display_plane_properties2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceDisplayPlaneProperties2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_display_plane_properties2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_display_mode_properties2_khr: unsafe {
                        unsafe extern "system" fn get_display_mode_properties2_khr(
                            _physical_device: PhysicalDevice,
                            _display: DisplayKHR,
                            _p_property_count: *mut u32,
                            _p_properties: *mut DisplayModeProperties2KHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_display_mode_properties2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDisplayModeProperties2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_display_mode_properties2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_display_plane_capabilities2_khr: unsafe {
                        unsafe extern "system" fn get_display_plane_capabilities2_khr(
                            _physical_device: PhysicalDevice,
                            _p_display_plane_info: *const DisplayPlaneInfo2KHR<'_>,
                            _p_capabilities: *mut DisplayPlaneCapabilities2KHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_display_plane_capabilities2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDisplayPlaneCapabilities2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_display_plane_capabilities2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_dedicated_allocation"]
    pub mod dedicated_allocation {
        use super::super::*;
        pub use {
            crate::vk::KHR_DEDICATED_ALLOCATION_NAME as NAME,
            crate::vk::KHR_DEDICATED_ALLOCATION_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_storage_buffer_storage_class"]
    pub mod storage_buffer_storage_class {
        use super::super::*;
        pub use {
            crate::vk::KHR_STORAGE_BUFFER_STORAGE_CLASS_NAME as NAME,
            crate::vk::KHR_STORAGE_BUFFER_STORAGE_CLASS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_relaxed_block_layout"]
    pub mod relaxed_block_layout {
        use super::super::*;
        pub use {
            crate::vk::KHR_RELAXED_BLOCK_LAYOUT_NAME as NAME,
            crate::vk::KHR_RELAXED_BLOCK_LAYOUT_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_get_memory_requirements2"]
    pub mod get_memory_requirements2 {
        use super::super::*;
        pub use {
            crate::vk::KHR_GET_MEMORY_REQUIREMENTS2_NAME as NAME,
            crate::vk::KHR_GET_MEMORY_REQUIREMENTS2_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_get_memory_requirements2 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_get_memory_requirements2 device-level function pointers"]
        pub struct DeviceFn {
            pub get_image_memory_requirements2_khr: PFN_vkGetImageMemoryRequirements2,
            pub get_buffer_memory_requirements2_khr: PFN_vkGetBufferMemoryRequirements2,
            pub get_image_sparse_memory_requirements2_khr: PFN_vkGetImageSparseMemoryRequirements2,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_image_memory_requirements2_khr: unsafe {
                        unsafe extern "system" fn get_image_memory_requirements2_khr(
                            _device: crate::vk::Device,
                            _p_info: *const ImageMemoryRequirementsInfo2<'_>,
                            _p_memory_requirements: *mut MemoryRequirements2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_image_memory_requirements2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetImageMemoryRequirements2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_image_memory_requirements2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_buffer_memory_requirements2_khr: unsafe {
                        unsafe extern "system" fn get_buffer_memory_requirements2_khr(
                            _device: crate::vk::Device,
                            _p_info: *const BufferMemoryRequirementsInfo2<'_>,
                            _p_memory_requirements: *mut MemoryRequirements2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_buffer_memory_requirements2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetBufferMemoryRequirements2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_buffer_memory_requirements2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_image_sparse_memory_requirements2_khr: unsafe {
                        unsafe extern "system" fn get_image_sparse_memory_requirements2_khr(
                            _device: crate::vk::Device,
                            _p_info: *const ImageSparseMemoryRequirementsInfo2<'_>,
                            _p_sparse_memory_requirement_count: *mut u32,
                            _p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_image_sparse_memory_requirements2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetImageSparseMemoryRequirements2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_image_sparse_memory_requirements2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_image_format_list"]
    pub mod image_format_list {
        use super::super::*;
        pub use {
            crate::vk::KHR_IMAGE_FORMAT_LIST_NAME as NAME,
            crate::vk::KHR_IMAGE_FORMAT_LIST_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_acceleration_structure"]
    pub mod acceleration_structure {
        use super::super::*;
        pub use {
            crate::vk::KHR_ACCELERATION_STRUCTURE_NAME as NAME,
            crate::vk::KHR_ACCELERATION_STRUCTURE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_acceleration_structure device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_acceleration_structure device-level function pointers"]
        pub struct DeviceFn {
            pub create_acceleration_structure_khr: PFN_vkCreateAccelerationStructureKHR,
            pub destroy_acceleration_structure_khr: PFN_vkDestroyAccelerationStructureKHR,
            pub cmd_build_acceleration_structures_khr: PFN_vkCmdBuildAccelerationStructuresKHR,
            pub cmd_build_acceleration_structures_indirect_khr:
                PFN_vkCmdBuildAccelerationStructuresIndirectKHR,
            pub build_acceleration_structures_khr: PFN_vkBuildAccelerationStructuresKHR,
            pub copy_acceleration_structure_khr: PFN_vkCopyAccelerationStructureKHR,
            pub copy_acceleration_structure_to_memory_khr:
                PFN_vkCopyAccelerationStructureToMemoryKHR,
            pub copy_memory_to_acceleration_structure_khr:
                PFN_vkCopyMemoryToAccelerationStructureKHR,
            pub write_acceleration_structures_properties_khr:
                PFN_vkWriteAccelerationStructuresPropertiesKHR,
            pub cmd_copy_acceleration_structure_khr: PFN_vkCmdCopyAccelerationStructureKHR,
            pub cmd_copy_acceleration_structure_to_memory_khr:
                PFN_vkCmdCopyAccelerationStructureToMemoryKHR,
            pub cmd_copy_memory_to_acceleration_structure_khr:
                PFN_vkCmdCopyMemoryToAccelerationStructureKHR,
            pub get_acceleration_structure_device_address_khr:
                PFN_vkGetAccelerationStructureDeviceAddressKHR,
            pub cmd_write_acceleration_structures_properties_khr:
                PFN_vkCmdWriteAccelerationStructuresPropertiesKHR,
            pub get_device_acceleration_structure_compatibility_khr:
                PFN_vkGetDeviceAccelerationStructureCompatibilityKHR,
            pub get_acceleration_structure_build_sizes_khr:
                PFN_vkGetAccelerationStructureBuildSizesKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_acceleration_structure_khr: unsafe {
                        unsafe extern "system" fn create_acceleration_structure_khr(
                            _device: crate::vk::Device,
                            _p_create_info: *const AccelerationStructureCreateInfoKHR<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_acceleration_structure: *mut AccelerationStructureKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_acceleration_structure_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCreateAccelerationStructureKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            create_acceleration_structure_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_acceleration_structure_khr: unsafe {
                        unsafe extern "system" fn destroy_acceleration_structure_khr(
                            _device: crate::vk::Device,
                            _acceleration_structure: AccelerationStructureKHR,
                            _p_allocator: *const AllocationCallbacks<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(destroy_acceleration_structure_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkDestroyAccelerationStructureKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_acceleration_structure_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_build_acceleration_structures_khr: unsafe {
                        unsafe extern "system" fn cmd_build_acceleration_structures_khr(
                            _command_buffer: CommandBuffer,
                            _info_count: u32,
                            _p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
                            _pp_build_range_infos : * const * const AccelerationStructureBuildRangeInfoKHR,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_build_acceleration_structures_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdBuildAccelerationStructuresKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_build_acceleration_structures_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_build_acceleration_structures_indirect_khr: unsafe {
                        unsafe extern "system" fn cmd_build_acceleration_structures_indirect_khr(
                            _command_buffer: CommandBuffer,
                            _info_count: u32,
                            _p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
                            _p_indirect_device_addresses: *const DeviceAddress,
                            _p_indirect_strides: *const u32,
                            _pp_max_primitive_counts: *const *const u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_build_acceleration_structures_indirect_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdBuildAccelerationStructuresIndirectKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_build_acceleration_structures_indirect_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    build_acceleration_structures_khr: unsafe {
                        unsafe extern "system" fn build_acceleration_structures_khr(
                            _device: crate::vk::Device,
                            _deferred_operation: DeferredOperationKHR,
                            _info_count: u32,
                            _p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
                            _pp_build_range_infos : * const * const AccelerationStructureBuildRangeInfoKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(build_acceleration_structures_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkBuildAccelerationStructuresKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            build_acceleration_structures_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    copy_acceleration_structure_khr: unsafe {
                        unsafe extern "system" fn copy_acceleration_structure_khr(
                            _device: crate::vk::Device,
                            _deferred_operation: DeferredOperationKHR,
                            _p_info: *const CopyAccelerationStructureInfoKHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(copy_acceleration_structure_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCopyAccelerationStructureKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            copy_acceleration_structure_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    copy_acceleration_structure_to_memory_khr: unsafe {
                        unsafe extern "system" fn copy_acceleration_structure_to_memory_khr(
                            _device: crate::vk::Device,
                            _deferred_operation: DeferredOperationKHR,
                            _p_info: *const CopyAccelerationStructureToMemoryInfoKHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(copy_acceleration_structure_to_memory_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCopyAccelerationStructureToMemoryKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            copy_acceleration_structure_to_memory_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    copy_memory_to_acceleration_structure_khr: unsafe {
                        unsafe extern "system" fn copy_memory_to_acceleration_structure_khr(
                            _device: crate::vk::Device,
                            _deferred_operation: DeferredOperationKHR,
                            _p_info: *const CopyMemoryToAccelerationStructureInfoKHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(copy_memory_to_acceleration_structure_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCopyMemoryToAccelerationStructureKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            copy_memory_to_acceleration_structure_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    write_acceleration_structures_properties_khr: unsafe {
                        unsafe extern "system" fn write_acceleration_structures_properties_khr(
                            _device: crate::vk::Device,
                            _acceleration_structure_count: u32,
                            _p_acceleration_structures: *const AccelerationStructureKHR,
                            _query_type: QueryType,
                            _data_size: usize,
                            _p_data: *mut c_void,
                            _stride: usize,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(write_acceleration_structures_properties_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkWriteAccelerationStructuresPropertiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            write_acceleration_structures_properties_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_copy_acceleration_structure_khr: unsafe {
                        unsafe extern "system" fn cmd_copy_acceleration_structure_khr(
                            _command_buffer: CommandBuffer,
                            _p_info: *const CopyAccelerationStructureInfoKHR<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_copy_acceleration_structure_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdCopyAccelerationStructureKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_copy_acceleration_structure_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_copy_acceleration_structure_to_memory_khr: unsafe {
                        unsafe extern "system" fn cmd_copy_acceleration_structure_to_memory_khr(
                            _command_buffer: CommandBuffer,
                            _p_info: *const CopyAccelerationStructureToMemoryInfoKHR<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_copy_acceleration_structure_to_memory_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdCopyAccelerationStructureToMemoryKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_copy_acceleration_structure_to_memory_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_copy_memory_to_acceleration_structure_khr: unsafe {
                        unsafe extern "system" fn cmd_copy_memory_to_acceleration_structure_khr(
                            _command_buffer: CommandBuffer,
                            _p_info: *const CopyMemoryToAccelerationStructureInfoKHR<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_copy_memory_to_acceleration_structure_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdCopyMemoryToAccelerationStructureKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_copy_memory_to_acceleration_structure_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_acceleration_structure_device_address_khr: unsafe {
                        unsafe extern "system" fn get_acceleration_structure_device_address_khr(
                            _device: crate::vk::Device,
                            _p_info: *const AccelerationStructureDeviceAddressInfoKHR<'_>,
                        ) -> DeviceAddress {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_acceleration_structure_device_address_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetAccelerationStructureDeviceAddressKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_acceleration_structure_device_address_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_write_acceleration_structures_properties_khr: unsafe {
                        unsafe extern "system" fn cmd_write_acceleration_structures_properties_khr(
                            _command_buffer: CommandBuffer,
                            _acceleration_structure_count: u32,
                            _p_acceleration_structures: *const AccelerationStructureKHR,
                            _query_type: QueryType,
                            _query_pool: QueryPool,
                            _first_query: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_write_acceleration_structures_properties_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdWriteAccelerationStructuresPropertiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_write_acceleration_structures_properties_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_device_acceleration_structure_compatibility_khr: unsafe {
                        unsafe extern "system" fn get_device_acceleration_structure_compatibility_khr(
                            _device: crate::vk::Device,
                            _p_version_info: *const AccelerationStructureVersionInfoKHR<'_>,
                            _p_compatibility: *mut AccelerationStructureCompatibilityKHR,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_device_acceleration_structure_compatibility_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeviceAccelerationStructureCompatibilityKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_device_acceleration_structure_compatibility_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_acceleration_structure_build_sizes_khr: unsafe {
                        unsafe extern "system" fn get_acceleration_structure_build_sizes_khr(
                            _device: crate::vk::Device,
                            _build_type: AccelerationStructureBuildTypeKHR,
                            _p_build_info: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
                            _p_max_primitive_counts: *const u32,
                            _p_size_info: *mut AccelerationStructureBuildSizesInfoKHR<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_acceleration_structure_build_sizes_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetAccelerationStructureBuildSizesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_acceleration_structure_build_sizes_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_ray_tracing_pipeline"]
    pub mod ray_tracing_pipeline {
        use super::super::*;
        pub use {
            crate::vk::KHR_RAY_TRACING_PIPELINE_NAME as NAME,
            crate::vk::KHR_RAY_TRACING_PIPELINE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_ray_tracing_pipeline device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_ray_tracing_pipeline device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_trace_rays_khr: PFN_vkCmdTraceRaysKHR,
            pub create_ray_tracing_pipelines_khr: PFN_vkCreateRayTracingPipelinesKHR,
            pub get_ray_tracing_shader_group_handles_khr: PFN_vkGetRayTracingShaderGroupHandlesKHR,
            pub get_ray_tracing_capture_replay_shader_group_handles_khr:
                PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR,
            pub cmd_trace_rays_indirect_khr: PFN_vkCmdTraceRaysIndirectKHR,
            pub get_ray_tracing_shader_group_stack_size_khr:
                PFN_vkGetRayTracingShaderGroupStackSizeKHR,
            pub cmd_set_ray_tracing_pipeline_stack_size_khr:
                PFN_vkCmdSetRayTracingPipelineStackSizeKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_trace_rays_khr: unsafe {
                        unsafe extern "system" fn cmd_trace_rays_khr(
                            _command_buffer: CommandBuffer,
                            _p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                            _p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                            _p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                            _p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                            _width: u32,
                            _height: u32,
                            _depth: u32,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_trace_rays_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdTraceRaysKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_trace_rays_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    create_ray_tracing_pipelines_khr: unsafe {
                        unsafe extern "system" fn create_ray_tracing_pipelines_khr(
                            _device: crate::vk::Device,
                            _deferred_operation: DeferredOperationKHR,
                            _pipeline_cache: PipelineCache,
                            _create_info_count: u32,
                            _p_create_infos: *const RayTracingPipelineCreateInfoKHR<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_pipelines: *mut Pipeline,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_ray_tracing_pipelines_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCreateRayTracingPipelinesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            create_ray_tracing_pipelines_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_ray_tracing_shader_group_handles_khr: unsafe {
                        unsafe extern "system" fn get_ray_tracing_shader_group_handles_khr(
                            _device: crate::vk::Device,
                            _pipeline: Pipeline,
                            _first_group: u32,
                            _group_count: u32,
                            _data_size: usize,
                            _p_data: *mut c_void,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_ray_tracing_shader_group_handles_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetRayTracingShaderGroupHandlesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_ray_tracing_shader_group_handles_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_ray_tracing_capture_replay_shader_group_handles_khr: unsafe {
                        unsafe extern "system" fn get_ray_tracing_capture_replay_shader_group_handles_khr(
                            _device: crate::vk::Device,
                            _pipeline: Pipeline,
                            _first_group: u32,
                            _group_count: u32,
                            _data_size: usize,
                            _p_data: *mut c_void,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_ray_tracing_capture_replay_shader_group_handles_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetRayTracingCaptureReplayShaderGroupHandlesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_ray_tracing_capture_replay_shader_group_handles_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_trace_rays_indirect_khr: unsafe {
                        unsafe extern "system" fn cmd_trace_rays_indirect_khr(
                            _command_buffer: CommandBuffer,
                            _p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                            _p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                            _p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                            _p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                            _indirect_device_address: DeviceAddress,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_trace_rays_indirect_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdTraceRaysIndirectKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_trace_rays_indirect_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_ray_tracing_shader_group_stack_size_khr: unsafe {
                        unsafe extern "system" fn get_ray_tracing_shader_group_stack_size_khr(
                            _device: crate::vk::Device,
                            _pipeline: Pipeline,
                            _group: u32,
                            _group_shader: ShaderGroupShaderKHR,
                        ) -> DeviceSize {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_ray_tracing_shader_group_stack_size_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetRayTracingShaderGroupStackSizeKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_ray_tracing_shader_group_stack_size_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_ray_tracing_pipeline_stack_size_khr: unsafe {
                        unsafe extern "system" fn cmd_set_ray_tracing_pipeline_stack_size_khr(
                            _command_buffer: CommandBuffer,
                            _pipeline_stack_size: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_ray_tracing_pipeline_stack_size_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetRayTracingPipelineStackSizeKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_ray_tracing_pipeline_stack_size_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_ray_query"]
    pub mod ray_query {
        use super::super::*;
        pub use {
            crate::vk::KHR_RAY_QUERY_NAME as NAME,
            crate::vk::KHR_RAY_QUERY_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_sampler_ycbcr_conversion"]
    pub mod sampler_ycbcr_conversion {
        use super::super::*;
        pub use {
            crate::vk::KHR_SAMPLER_YCBCR_CONVERSION_NAME as NAME,
            crate::vk::KHR_SAMPLER_YCBCR_CONVERSION_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_sampler_ycbcr_conversion device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_sampler_ycbcr_conversion device-level function pointers"]
        pub struct DeviceFn {
            pub create_sampler_ycbcr_conversion_khr: PFN_vkCreateSamplerYcbcrConversion,
            pub destroy_sampler_ycbcr_conversion_khr: PFN_vkDestroySamplerYcbcrConversion,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_sampler_ycbcr_conversion_khr: unsafe {
                        unsafe extern "system" fn create_sampler_ycbcr_conversion_khr(
                            _device: crate::vk::Device,
                            _p_create_info: *const SamplerYcbcrConversionCreateInfo<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_ycbcr_conversion: *mut SamplerYcbcrConversion,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_sampler_ycbcr_conversion_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCreateSamplerYcbcrConversionKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            create_sampler_ycbcr_conversion_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_sampler_ycbcr_conversion_khr: unsafe {
                        unsafe extern "system" fn destroy_sampler_ycbcr_conversion_khr(
                            _device: crate::vk::Device,
                            _ycbcr_conversion: SamplerYcbcrConversion,
                            _p_allocator: *const AllocationCallbacks<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(destroy_sampler_ycbcr_conversion_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkDestroySamplerYcbcrConversionKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_sampler_ycbcr_conversion_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_bind_memory2"]
    pub mod bind_memory2 {
        use super::super::*;
        pub use {
            crate::vk::KHR_BIND_MEMORY2_NAME as NAME,
            crate::vk::KHR_BIND_MEMORY2_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_bind_memory2 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_bind_memory2 device-level function pointers"]
        pub struct DeviceFn {
            pub bind_buffer_memory2_khr: PFN_vkBindBufferMemory2,
            pub bind_image_memory2_khr: PFN_vkBindImageMemory2,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    bind_buffer_memory2_khr: unsafe {
                        unsafe extern "system" fn bind_buffer_memory2_khr(
                            _device: crate::vk::Device,
                            _bind_info_count: u32,
                            _p_bind_infos: *const BindBufferMemoryInfo<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(bind_buffer_memory2_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkBindBufferMemory2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            bind_buffer_memory2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    bind_image_memory2_khr: unsafe {
                        unsafe extern "system" fn bind_image_memory2_khr(
                            _device: crate::vk::Device,
                            _bind_info_count: u32,
                            _p_bind_infos: *const BindImageMemoryInfo<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(bind_image_memory2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkBindImageMemory2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            bind_image_memory2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_portability_subset"]
    pub mod portability_subset {
        use super::super::*;
        pub use {
            crate::vk::KHR_PORTABILITY_SUBSET_NAME as NAME,
            crate::vk::KHR_PORTABILITY_SUBSET_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_maintenance3"]
    pub mod maintenance3 {
        use super::super::*;
        pub use {
            crate::vk::KHR_MAINTENANCE3_NAME as NAME,
            crate::vk::KHR_MAINTENANCE3_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_maintenance3 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_maintenance3 device-level function pointers"]
        pub struct DeviceFn {
            pub get_descriptor_set_layout_support_khr: PFN_vkGetDescriptorSetLayoutSupport,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_descriptor_set_layout_support_khr: unsafe {
                        unsafe extern "system" fn get_descriptor_set_layout_support_khr(
                            _device: crate::vk::Device,
                            _p_create_info: *const DescriptorSetLayoutCreateInfo<'_>,
                            _p_support: *mut DescriptorSetLayoutSupport<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_descriptor_set_layout_support_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDescriptorSetLayoutSupportKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_descriptor_set_layout_support_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_draw_indirect_count"]
    pub mod draw_indirect_count {
        use super::super::*;
        pub use {
            crate::vk::KHR_DRAW_INDIRECT_COUNT_NAME as NAME,
            crate::vk::KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_draw_indirect_count device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_draw_indirect_count device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_draw_indirect_count_khr: PFN_vkCmdDrawIndirectCount,
            pub cmd_draw_indexed_indirect_count_khr: PFN_vkCmdDrawIndexedIndirectCount,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_draw_indirect_count_khr: unsafe {
                        unsafe extern "system" fn cmd_draw_indirect_count_khr(
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
                                stringify!(cmd_draw_indirect_count_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndirectCountKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_draw_indirect_count_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_draw_indexed_indirect_count_khr: unsafe {
                        unsafe extern "system" fn cmd_draw_indexed_indirect_count_khr(
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
                                stringify!(cmd_draw_indexed_indirect_count_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdDrawIndexedIndirectCountKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_draw_indexed_indirect_count_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_shader_subgroup_extended_types"]
    pub mod shader_subgroup_extended_types {
        use super::super::*;
        pub use {
            crate::vk::KHR_SHADER_SUBGROUP_EXTENDED_TYPES_NAME as NAME,
            crate::vk::KHR_SHADER_SUBGROUP_EXTENDED_TYPES_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_8bit_storage"]
    pub mod _8bit_storage {
        use super::super::*;
        pub use {
            crate::vk::KHR_8BIT_STORAGE_NAME as NAME,
            crate::vk::KHR_8BIT_STORAGE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_shader_atomic_int64"]
    pub mod shader_atomic_int64 {
        use super::super::*;
        pub use {
            crate::vk::KHR_SHADER_ATOMIC_INT64_NAME as NAME,
            crate::vk::KHR_SHADER_ATOMIC_INT64_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_shader_clock"]
    pub mod shader_clock {
        use super::super::*;
        pub use {
            crate::vk::KHR_SHADER_CLOCK_NAME as NAME,
            crate::vk::KHR_SHADER_CLOCK_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_video_decode_h265"]
    pub mod video_decode_h265 {
        use super::super::*;
        pub use {
            crate::vk::KHR_VIDEO_DECODE_H265_NAME as NAME,
            crate::vk::KHR_VIDEO_DECODE_H265_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_global_priority"]
    pub mod global_priority {
        use super::super::*;
        pub use {
            crate::vk::KHR_GLOBAL_PRIORITY_NAME as NAME,
            crate::vk::KHR_GLOBAL_PRIORITY_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_driver_properties"]
    pub mod driver_properties {
        use super::super::*;
        pub use {
            crate::vk::KHR_DRIVER_PROPERTIES_NAME as NAME,
            crate::vk::KHR_DRIVER_PROPERTIES_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_shader_float_controls"]
    pub mod shader_float_controls {
        use super::super::*;
        pub use {
            crate::vk::KHR_SHADER_FLOAT_CONTROLS_NAME as NAME,
            crate::vk::KHR_SHADER_FLOAT_CONTROLS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_depth_stencil_resolve"]
    pub mod depth_stencil_resolve {
        use super::super::*;
        pub use {
            crate::vk::KHR_DEPTH_STENCIL_RESOLVE_NAME as NAME,
            crate::vk::KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_swapchain_mutable_format"]
    pub mod swapchain_mutable_format {
        use super::super::*;
        pub use {
            crate::vk::KHR_SWAPCHAIN_MUTABLE_FORMAT_NAME as NAME,
            crate::vk::KHR_SWAPCHAIN_MUTABLE_FORMAT_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_timeline_semaphore"]
    pub mod timeline_semaphore {
        use super::super::*;
        pub use {
            crate::vk::KHR_TIMELINE_SEMAPHORE_NAME as NAME,
            crate::vk::KHR_TIMELINE_SEMAPHORE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_timeline_semaphore device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_timeline_semaphore device-level function pointers"]
        pub struct DeviceFn {
            pub get_semaphore_counter_value_khr: PFN_vkGetSemaphoreCounterValue,
            pub wait_semaphores_khr: PFN_vkWaitSemaphores,
            pub signal_semaphore_khr: PFN_vkSignalSemaphore,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_semaphore_counter_value_khr: unsafe {
                        unsafe extern "system" fn get_semaphore_counter_value_khr(
                            _device: crate::vk::Device,
                            _semaphore: Semaphore,
                            _p_value: *mut u64,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_semaphore_counter_value_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetSemaphoreCounterValueKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_semaphore_counter_value_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    wait_semaphores_khr: unsafe {
                        unsafe extern "system" fn wait_semaphores_khr(
                            _device: crate::vk::Device,
                            _p_wait_info: *const SemaphoreWaitInfo<'_>,
                            _timeout: u64,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(wait_semaphores_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkWaitSemaphoresKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            wait_semaphores_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    signal_semaphore_khr: unsafe {
                        unsafe extern "system" fn signal_semaphore_khr(
                            _device: crate::vk::Device,
                            _p_signal_info: *const SemaphoreSignalInfo<'_>,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(signal_semaphore_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkSignalSemaphoreKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            signal_semaphore_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_vulkan_memory_model"]
    pub mod vulkan_memory_model {
        use super::super::*;
        pub use {
            crate::vk::KHR_VULKAN_MEMORY_MODEL_NAME as NAME,
            crate::vk::KHR_VULKAN_MEMORY_MODEL_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_shader_terminate_invocation"]
    pub mod shader_terminate_invocation {
        use super::super::*;
        pub use {
            crate::vk::KHR_SHADER_TERMINATE_INVOCATION_NAME as NAME,
            crate::vk::KHR_SHADER_TERMINATE_INVOCATION_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_fragment_shading_rate"]
    pub mod fragment_shading_rate {
        use super::super::*;
        pub use {
            crate::vk::KHR_FRAGMENT_SHADING_RATE_NAME as NAME,
            crate::vk::KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_fragment_shading_rate instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_fragment_shading_rate instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_fragment_shading_rates_khr:
                PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_fragment_shading_rates_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_fragment_shading_rates_khr(
                            _physical_device: PhysicalDevice,
                            _p_fragment_shading_rate_count: *mut u32,
                            _p_fragment_shading_rates: *mut PhysicalDeviceFragmentShadingRateKHR<
                                '_,
                            >,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_fragment_shading_rates_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceFragmentShadingRatesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_fragment_shading_rates_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
        #[doc = "VK_KHR_fragment_shading_rate device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_fragment_shading_rate device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_set_fragment_shading_rate_khr: PFN_vkCmdSetFragmentShadingRateKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_set_fragment_shading_rate_khr: unsafe {
                        unsafe extern "system" fn cmd_set_fragment_shading_rate_khr(
                            _command_buffer: CommandBuffer,
                            _p_fragment_size: *const Extent2D,
                            _combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2usize],
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_fragment_shading_rate_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetFragmentShadingRateKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_fragment_shading_rate_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_dynamic_rendering_local_read"]
    pub mod dynamic_rendering_local_read {
        use super::super::*;
        pub use {
            crate::vk::KHR_DYNAMIC_RENDERING_LOCAL_READ_NAME as NAME,
            crate::vk::KHR_DYNAMIC_RENDERING_LOCAL_READ_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_dynamic_rendering_local_read device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_dynamic_rendering_local_read device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_set_rendering_attachment_locations_khr:
                PFN_vkCmdSetRenderingAttachmentLocationsKHR,
            pub cmd_set_rendering_input_attachment_indices_khr:
                PFN_vkCmdSetRenderingInputAttachmentIndicesKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_set_rendering_attachment_locations_khr: unsafe {
                        unsafe extern "system" fn cmd_set_rendering_attachment_locations_khr(
                            _command_buffer: CommandBuffer,
                            _p_location_info: *const RenderingAttachmentLocationInfoKHR<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_rendering_attachment_locations_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetRenderingAttachmentLocationsKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_rendering_attachment_locations_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_rendering_input_attachment_indices_khr: unsafe {
                        unsafe extern "system" fn cmd_set_rendering_input_attachment_indices_khr(
                            _command_buffer: CommandBuffer,
                            _p_location_info: *const RenderingInputAttachmentIndexInfoKHR<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_rendering_input_attachment_indices_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetRenderingInputAttachmentIndicesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_rendering_input_attachment_indices_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_shader_quad_control"]
    pub mod shader_quad_control {
        use super::super::*;
        pub use {
            crate::vk::KHR_SHADER_QUAD_CONTROL_NAME as NAME,
            crate::vk::KHR_SHADER_QUAD_CONTROL_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_spirv_1_4"]
    pub mod spirv_1_4 {
        use super::super::*;
        pub use {
            crate::vk::KHR_SPIRV_1_4_NAME as NAME,
            crate::vk::KHR_SPIRV_1_4_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_surface_protected_capabilities"]
    pub mod surface_protected_capabilities {
        use super::super::*;
        pub use {
            crate::vk::KHR_SURFACE_PROTECTED_CAPABILITIES_NAME as NAME,
            crate::vk::KHR_SURFACE_PROTECTED_CAPABILITIES_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_separate_depth_stencil_layouts"]
    pub mod separate_depth_stencil_layouts {
        use super::super::*;
        pub use {
            crate::vk::KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_NAME as NAME,
            crate::vk::KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_present_wait"]
    pub mod present_wait {
        use super::super::*;
        pub use {
            crate::vk::KHR_PRESENT_WAIT_NAME as NAME,
            crate::vk::KHR_PRESENT_WAIT_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_present_wait device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_present_wait device-level function pointers"]
        pub struct DeviceFn {
            pub wait_for_present_khr: PFN_vkWaitForPresentKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    wait_for_present_khr: unsafe {
                        unsafe extern "system" fn wait_for_present_khr(
                            _device: crate::vk::Device,
                            _swapchain: SwapchainKHR,
                            _present_id: u64,
                            _timeout: u64,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(wait_for_present_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkWaitForPresentKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            wait_for_present_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_uniform_buffer_standard_layout"]
    pub mod uniform_buffer_standard_layout {
        use super::super::*;
        pub use {
            crate::vk::KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_NAME as NAME,
            crate::vk::KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_buffer_device_address"]
    pub mod buffer_device_address {
        use super::super::*;
        pub use {
            crate::vk::KHR_BUFFER_DEVICE_ADDRESS_NAME as NAME,
            crate::vk::KHR_BUFFER_DEVICE_ADDRESS_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_buffer_device_address device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_buffer_device_address device-level function pointers"]
        pub struct DeviceFn {
            pub get_buffer_device_address_khr: PFN_vkGetBufferDeviceAddress,
            pub get_buffer_opaque_capture_address_khr: PFN_vkGetBufferOpaqueCaptureAddress,
            pub get_device_memory_opaque_capture_address_khr:
                PFN_vkGetDeviceMemoryOpaqueCaptureAddress,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_buffer_device_address_khr: unsafe {
                        unsafe extern "system" fn get_buffer_device_address_khr(
                            _device: crate::vk::Device,
                            _p_info: *const BufferDeviceAddressInfo<'_>,
                        ) -> DeviceAddress {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_buffer_device_address_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetBufferDeviceAddressKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_buffer_device_address_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_buffer_opaque_capture_address_khr: unsafe {
                        unsafe extern "system" fn get_buffer_opaque_capture_address_khr(
                            _device: crate::vk::Device,
                            _p_info: *const BufferDeviceAddressInfo<'_>,
                        ) -> u64 {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_buffer_opaque_capture_address_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetBufferOpaqueCaptureAddressKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_buffer_opaque_capture_address_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_device_memory_opaque_capture_address_khr: unsafe {
                        unsafe extern "system" fn get_device_memory_opaque_capture_address_khr(
                            _device: crate::vk::Device,
                            _p_info: *const DeviceMemoryOpaqueCaptureAddressInfo<'_>,
                        ) -> u64 {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_device_memory_opaque_capture_address_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeviceMemoryOpaqueCaptureAddressKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_device_memory_opaque_capture_address_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_deferred_host_operations"]
    pub mod deferred_host_operations {
        use super::super::*;
        pub use {
            crate::vk::KHR_DEFERRED_HOST_OPERATIONS_NAME as NAME,
            crate::vk::KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_deferred_host_operations device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_deferred_host_operations device-level function pointers"]
        pub struct DeviceFn {
            pub create_deferred_operation_khr: PFN_vkCreateDeferredOperationKHR,
            pub destroy_deferred_operation_khr: PFN_vkDestroyDeferredOperationKHR,
            pub get_deferred_operation_max_concurrency_khr:
                PFN_vkGetDeferredOperationMaxConcurrencyKHR,
            pub get_deferred_operation_result_khr: PFN_vkGetDeferredOperationResultKHR,
            pub deferred_operation_join_khr: PFN_vkDeferredOperationJoinKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_deferred_operation_khr: unsafe {
                        unsafe extern "system" fn create_deferred_operation_khr(
                            _device: crate::vk::Device,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_deferred_operation: *mut DeferredOperationKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_deferred_operation_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateDeferredOperationKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_deferred_operation_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_deferred_operation_khr: unsafe {
                        unsafe extern "system" fn destroy_deferred_operation_khr(
                            _device: crate::vk::Device,
                            _operation: DeferredOperationKHR,
                            _p_allocator: *const AllocationCallbacks<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(destroy_deferred_operation_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkDestroyDeferredOperationKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_deferred_operation_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_deferred_operation_max_concurrency_khr: unsafe {
                        unsafe extern "system" fn get_deferred_operation_max_concurrency_khr(
                            _device: crate::vk::Device,
                            _operation: DeferredOperationKHR,
                        ) -> u32 {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_deferred_operation_max_concurrency_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeferredOperationMaxConcurrencyKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_deferred_operation_max_concurrency_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_deferred_operation_result_khr: unsafe {
                        unsafe extern "system" fn get_deferred_operation_result_khr(
                            _device: crate::vk::Device,
                            _operation: DeferredOperationKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_deferred_operation_result_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeferredOperationResultKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_deferred_operation_result_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    deferred_operation_join_khr: unsafe {
                        unsafe extern "system" fn deferred_operation_join_khr(
                            _device: crate::vk::Device,
                            _operation: DeferredOperationKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(deferred_operation_join_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkDeferredOperationJoinKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            deferred_operation_join_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_pipeline_executable_properties"]
    pub mod pipeline_executable_properties {
        use super::super::*;
        pub use {
            crate::vk::KHR_PIPELINE_EXECUTABLE_PROPERTIES_NAME as NAME,
            crate::vk::KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_pipeline_executable_properties device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_pipeline_executable_properties device-level function pointers"]
        pub struct DeviceFn {
            pub get_pipeline_executable_properties_khr: PFN_vkGetPipelineExecutablePropertiesKHR,
            pub get_pipeline_executable_statistics_khr: PFN_vkGetPipelineExecutableStatisticsKHR,
            pub get_pipeline_executable_internal_representations_khr:
                PFN_vkGetPipelineExecutableInternalRepresentationsKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_pipeline_executable_properties_khr: unsafe {
                        unsafe extern "system" fn get_pipeline_executable_properties_khr(
                            _device: crate::vk::Device,
                            _p_pipeline_info: *const PipelineInfoKHR<'_>,
                            _p_executable_count: *mut u32,
                            _p_properties: *mut PipelineExecutablePropertiesKHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_pipeline_executable_properties_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPipelineExecutablePropertiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_pipeline_executable_properties_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_pipeline_executable_statistics_khr: unsafe {
                        unsafe extern "system" fn get_pipeline_executable_statistics_khr(
                            _device: crate::vk::Device,
                            _p_executable_info: *const PipelineExecutableInfoKHR<'_>,
                            _p_statistic_count: *mut u32,
                            _p_statistics: *mut PipelineExecutableStatisticKHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_pipeline_executable_statistics_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPipelineExecutableStatisticsKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_pipeline_executable_statistics_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_pipeline_executable_internal_representations_khr: unsafe {
                        unsafe extern "system" fn get_pipeline_executable_internal_representations_khr(
                            _device: crate::vk::Device,
                            _p_executable_info: *const PipelineExecutableInfoKHR<'_>,
                            _p_internal_representation_count: *mut u32,
                            _p_internal_representations : * mut PipelineExecutableInternalRepresentationKHR < '_ >,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_pipeline_executable_internal_representations_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPipelineExecutableInternalRepresentationsKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_pipeline_executable_internal_representations_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_map_memory2"]
    pub mod map_memory2 {
        use super::super::*;
        pub use {
            crate::vk::KHR_MAP_MEMORY2_NAME as NAME,
            crate::vk::KHR_MAP_MEMORY2_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_map_memory2 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_map_memory2 device-level function pointers"]
        pub struct DeviceFn {
            pub map_memory2_khr: PFN_vkMapMemory2KHR,
            pub unmap_memory2_khr: PFN_vkUnmapMemory2KHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    map_memory2_khr: unsafe {
                        unsafe extern "system" fn map_memory2_khr(
                            _device: crate::vk::Device,
                            _p_memory_map_info: *const MemoryMapInfoKHR<'_>,
                            _pp_data: *mut *mut c_void,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(map_memory2_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkMapMemory2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            map_memory2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    unmap_memory2_khr: unsafe {
                        unsafe extern "system" fn unmap_memory2_khr(
                            _device: crate::vk::Device,
                            _p_memory_unmap_info: *const MemoryUnmapInfoKHR<'_>,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(unmap_memory2_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkUnmapMemory2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            unmap_memory2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_shader_integer_dot_product"]
    pub mod shader_integer_dot_product {
        use super::super::*;
        pub use {
            crate::vk::KHR_SHADER_INTEGER_DOT_PRODUCT_NAME as NAME,
            crate::vk::KHR_SHADER_INTEGER_DOT_PRODUCT_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_pipeline_library"]
    pub mod pipeline_library {
        use super::super::*;
        pub use {
            crate::vk::KHR_PIPELINE_LIBRARY_NAME as NAME,
            crate::vk::KHR_PIPELINE_LIBRARY_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_shader_non_semantic_info"]
    pub mod shader_non_semantic_info {
        use super::super::*;
        pub use {
            crate::vk::KHR_SHADER_NON_SEMANTIC_INFO_NAME as NAME,
            crate::vk::KHR_SHADER_NON_SEMANTIC_INFO_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_present_id"]
    pub mod present_id {
        use super::super::*;
        pub use {
            crate::vk::KHR_PRESENT_ID_NAME as NAME,
            crate::vk::KHR_PRESENT_ID_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_video_encode_queue"]
    pub mod video_encode_queue {
        use super::super::*;
        pub use {
            crate::vk::KHR_VIDEO_ENCODE_QUEUE_NAME as NAME,
            crate::vk::KHR_VIDEO_ENCODE_QUEUE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_video_encode_queue instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_video_encode_queue instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_video_encode_quality_level_properties_khr:
                PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_video_encode_quality_level_properties_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_video_encode_quality_level_properties_khr(
                            _physical_device: PhysicalDevice,
                            _p_quality_level_info : * const PhysicalDeviceVideoEncodeQualityLevelInfoKHR < '_ >,
                            _p_quality_level_properties: *mut VideoEncodeQualityLevelPropertiesKHR<
                                '_,
                            >,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(
                                    get_physical_device_video_encode_quality_level_properties_khr
                                )
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_video_encode_quality_level_properties_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
        #[doc = "VK_KHR_video_encode_queue device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_video_encode_queue device-level function pointers"]
        pub struct DeviceFn {
            pub get_encoded_video_session_parameters_khr: PFN_vkGetEncodedVideoSessionParametersKHR,
            pub cmd_encode_video_khr: PFN_vkCmdEncodeVideoKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_encoded_video_session_parameters_khr: unsafe {
                        unsafe extern "system" fn get_encoded_video_session_parameters_khr(
                            _device: crate::vk::Device,
                            _p_video_session_parameters_info : * const VideoEncodeSessionParametersGetInfoKHR < '_ >,
                            _p_feedback_info: *mut VideoEncodeSessionParametersFeedbackInfoKHR<'_>,
                            _p_data_size: *mut usize,
                            _p_data: *mut c_void,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_encoded_video_session_parameters_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetEncodedVideoSessionParametersKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_encoded_video_session_parameters_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_encode_video_khr: unsafe {
                        unsafe extern "system" fn cmd_encode_video_khr(
                            _command_buffer: CommandBuffer,
                            _p_encode_info: *const VideoEncodeInfoKHR<'_>,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_encode_video_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdEncodeVideoKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_encode_video_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_synchronization2"]
    pub mod synchronization2 {
        use super::super::*;
        pub use {
            crate::vk::KHR_SYNCHRONIZATION2_NAME as NAME,
            crate::vk::KHR_SYNCHRONIZATION2_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_synchronization2 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_synchronization2 device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_set_event2_khr: PFN_vkCmdSetEvent2,
            pub cmd_reset_event2_khr: PFN_vkCmdResetEvent2,
            pub cmd_wait_events2_khr: PFN_vkCmdWaitEvents2,
            pub cmd_pipeline_barrier2_khr: PFN_vkCmdPipelineBarrier2,
            pub cmd_write_timestamp2_khr: PFN_vkCmdWriteTimestamp2,
            pub queue_submit2_khr: PFN_vkQueueSubmit2,
            pub cmd_write_buffer_marker2_amd: PFN_vkCmdWriteBufferMarker2AMD,
            pub get_queue_checkpoint_data2_nv: PFN_vkGetQueueCheckpointData2NV,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_set_event2_khr: unsafe {
                        unsafe extern "system" fn cmd_set_event2_khr(
                            _command_buffer: CommandBuffer,
                            _event: Event,
                            _p_dependency_info: *const DependencyInfo<'_>,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_set_event2_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetEvent2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_event2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_reset_event2_khr: unsafe {
                        unsafe extern "system" fn cmd_reset_event2_khr(
                            _command_buffer: CommandBuffer,
                            _event: Event,
                            _stage_mask: PipelineStageFlags2,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_reset_event2_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdResetEvent2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_reset_event2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_wait_events2_khr: unsafe {
                        unsafe extern "system" fn cmd_wait_events2_khr(
                            _command_buffer: CommandBuffer,
                            _event_count: u32,
                            _p_events: *const Event,
                            _p_dependency_infos: *const DependencyInfo<'_>,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_wait_events2_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdWaitEvents2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_wait_events2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_pipeline_barrier2_khr: unsafe {
                        unsafe extern "system" fn cmd_pipeline_barrier2_khr(
                            _command_buffer: CommandBuffer,
                            _p_dependency_info: *const DependencyInfo<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_pipeline_barrier2_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdPipelineBarrier2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_pipeline_barrier2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_write_timestamp2_khr: unsafe {
                        unsafe extern "system" fn cmd_write_timestamp2_khr(
                            _command_buffer: CommandBuffer,
                            _stage: PipelineStageFlags2,
                            _query_pool: QueryPool,
                            _query: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_write_timestamp2_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdWriteTimestamp2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_write_timestamp2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    queue_submit2_khr: unsafe {
                        unsafe extern "system" fn queue_submit2_khr(
                            _queue: Queue,
                            _submit_count: u32,
                            _p_submits: *const SubmitInfo2<'_>,
                            _fence: Fence,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(queue_submit2_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkQueueSubmit2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            queue_submit2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_write_buffer_marker2_amd: unsafe {
                        unsafe extern "system" fn cmd_write_buffer_marker2_amd(
                            _command_buffer: CommandBuffer,
                            _stage: PipelineStageFlags2,
                            _dst_buffer: Buffer,
                            _dst_offset: DeviceSize,
                            _marker: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_write_buffer_marker2_amd)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdWriteBufferMarker2AMD\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_write_buffer_marker2_amd
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_queue_checkpoint_data2_nv: unsafe {
                        unsafe extern "system" fn get_queue_checkpoint_data2_nv(
                            _queue: Queue,
                            _p_checkpoint_data_count: *mut u32,
                            _p_checkpoint_data: *mut CheckpointData2NV<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_queue_checkpoint_data2_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetQueueCheckpointData2NV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_queue_checkpoint_data2_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_fragment_shader_barycentric"]
    pub mod fragment_shader_barycentric {
        use super::super::*;
        pub use {
            crate::vk::KHR_FRAGMENT_SHADER_BARYCENTRIC_NAME as NAME,
            crate::vk::KHR_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_shader_subgroup_uniform_control_flow"]
    pub mod shader_subgroup_uniform_control_flow {
        use super::super::*;
        pub use {
            crate::vk::KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_NAME as NAME,
            crate::vk::KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_zero_initialize_workgroup_memory"]
    pub mod zero_initialize_workgroup_memory {
        use super::super::*;
        pub use {
            crate::vk::KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_NAME as NAME,
            crate::vk::KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_workgroup_memory_explicit_layout"]
    pub mod workgroup_memory_explicit_layout {
        use super::super::*;
        pub use {
            crate::vk::KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_NAME as NAME,
            crate::vk::KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_copy_commands2"]
    pub mod copy_commands2 {
        use super::super::*;
        pub use {
            crate::vk::KHR_COPY_COMMANDS2_NAME as NAME,
            crate::vk::KHR_COPY_COMMANDS2_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_copy_commands2 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_copy_commands2 device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_copy_buffer2_khr: PFN_vkCmdCopyBuffer2,
            pub cmd_copy_image2_khr: PFN_vkCmdCopyImage2,
            pub cmd_copy_buffer_to_image2_khr: PFN_vkCmdCopyBufferToImage2,
            pub cmd_copy_image_to_buffer2_khr: PFN_vkCmdCopyImageToBuffer2,
            pub cmd_blit_image2_khr: PFN_vkCmdBlitImage2,
            pub cmd_resolve_image2_khr: PFN_vkCmdResolveImage2,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_copy_buffer2_khr: unsafe {
                        unsafe extern "system" fn cmd_copy_buffer2_khr(
                            _command_buffer: CommandBuffer,
                            _p_copy_buffer_info: *const CopyBufferInfo2<'_>,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_copy_buffer2_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyBuffer2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_copy_buffer2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_copy_image2_khr: unsafe {
                        unsafe extern "system" fn cmd_copy_image2_khr(
                            _command_buffer: CommandBuffer,
                            _p_copy_image_info: *const CopyImageInfo2<'_>,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_copy_image2_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyImage2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_copy_image2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_copy_buffer_to_image2_khr: unsafe {
                        unsafe extern "system" fn cmd_copy_buffer_to_image2_khr(
                            _command_buffer: CommandBuffer,
                            _p_copy_buffer_to_image_info: *const CopyBufferToImageInfo2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_copy_buffer_to_image2_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyBufferToImage2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_copy_buffer_to_image2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_copy_image_to_buffer2_khr: unsafe {
                        unsafe extern "system" fn cmd_copy_image_to_buffer2_khr(
                            _command_buffer: CommandBuffer,
                            _p_copy_image_to_buffer_info: *const CopyImageToBufferInfo2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_copy_image_to_buffer2_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyImageToBuffer2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_copy_image_to_buffer2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_blit_image2_khr: unsafe {
                        unsafe extern "system" fn cmd_blit_image2_khr(
                            _command_buffer: CommandBuffer,
                            _p_blit_image_info: *const BlitImageInfo2<'_>,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_blit_image2_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdBlitImage2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_blit_image2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_resolve_image2_khr: unsafe {
                        unsafe extern "system" fn cmd_resolve_image2_khr(
                            _command_buffer: CommandBuffer,
                            _p_resolve_image_info: *const ResolveImageInfo2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_resolve_image2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdResolveImage2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_resolve_image2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_format_feature_flags2"]
    pub mod format_feature_flags2 {
        use super::super::*;
        pub use {
            crate::vk::KHR_FORMAT_FEATURE_FLAGS2_NAME as NAME,
            crate::vk::KHR_FORMAT_FEATURE_FLAGS2_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_ray_tracing_maintenance1"]
    pub mod ray_tracing_maintenance1 {
        use super::super::*;
        pub use {
            crate::vk::KHR_RAY_TRACING_MAINTENANCE1_NAME as NAME,
            crate::vk::KHR_RAY_TRACING_MAINTENANCE1_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_ray_tracing_maintenance1 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_ray_tracing_maintenance1 device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_trace_rays_indirect2_khr: PFN_vkCmdTraceRaysIndirect2KHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_trace_rays_indirect2_khr: unsafe {
                        unsafe extern "system" fn cmd_trace_rays_indirect2_khr(
                            _command_buffer: CommandBuffer,
                            _indirect_device_address: DeviceAddress,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_trace_rays_indirect2_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdTraceRaysIndirect2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_trace_rays_indirect2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_portability_enumeration"]
    pub mod portability_enumeration {
        use super::super::*;
        pub use {
            crate::vk::KHR_PORTABILITY_ENUMERATION_NAME as NAME,
            crate::vk::KHR_PORTABILITY_ENUMERATION_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_maintenance4"]
    pub mod maintenance4 {
        use super::super::*;
        pub use {
            crate::vk::KHR_MAINTENANCE4_NAME as NAME,
            crate::vk::KHR_MAINTENANCE4_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_maintenance4 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_maintenance4 device-level function pointers"]
        pub struct DeviceFn {
            pub get_device_buffer_memory_requirements_khr: PFN_vkGetDeviceBufferMemoryRequirements,
            pub get_device_image_memory_requirements_khr: PFN_vkGetDeviceImageMemoryRequirements,
            pub get_device_image_sparse_memory_requirements_khr:
                PFN_vkGetDeviceImageSparseMemoryRequirements,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_device_buffer_memory_requirements_khr: unsafe {
                        unsafe extern "system" fn get_device_buffer_memory_requirements_khr(
                            _device: crate::vk::Device,
                            _p_info: *const DeviceBufferMemoryRequirements<'_>,
                            _p_memory_requirements: *mut MemoryRequirements2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_device_buffer_memory_requirements_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeviceBufferMemoryRequirementsKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_device_buffer_memory_requirements_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_device_image_memory_requirements_khr: unsafe {
                        unsafe extern "system" fn get_device_image_memory_requirements_khr(
                            _device: crate::vk::Device,
                            _p_info: *const DeviceImageMemoryRequirements<'_>,
                            _p_memory_requirements: *mut MemoryRequirements2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_device_image_memory_requirements_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeviceImageMemoryRequirementsKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_device_image_memory_requirements_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_device_image_sparse_memory_requirements_khr: unsafe {
                        unsafe extern "system" fn get_device_image_sparse_memory_requirements_khr(
                            _device: crate::vk::Device,
                            _p_info: *const DeviceImageMemoryRequirements<'_>,
                            _p_sparse_memory_requirement_count: *mut u32,
                            _p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_device_image_sparse_memory_requirements_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeviceImageSparseMemoryRequirementsKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_device_image_sparse_memory_requirements_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_shader_subgroup_rotate"]
    pub mod shader_subgroup_rotate {
        use super::super::*;
        pub use {
            crate::vk::KHR_SHADER_SUBGROUP_ROTATE_NAME as NAME,
            crate::vk::KHR_SHADER_SUBGROUP_ROTATE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_shader_maximal_reconvergence"]
    pub mod shader_maximal_reconvergence {
        use super::super::*;
        pub use {
            crate::vk::KHR_SHADER_MAXIMAL_RECONVERGENCE_NAME as NAME,
            crate::vk::KHR_SHADER_MAXIMAL_RECONVERGENCE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_maintenance5"]
    pub mod maintenance5 {
        use super::super::*;
        pub use {
            crate::vk::KHR_MAINTENANCE5_NAME as NAME,
            crate::vk::KHR_MAINTENANCE5_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_maintenance5 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_maintenance5 device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_bind_index_buffer2_khr: PFN_vkCmdBindIndexBuffer2KHR,
            pub get_rendering_area_granularity_khr: PFN_vkGetRenderingAreaGranularityKHR,
            pub get_device_image_subresource_layout_khr: PFN_vkGetDeviceImageSubresourceLayoutKHR,
            pub get_image_subresource_layout2_khr: PFN_vkGetImageSubresourceLayout2KHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_bind_index_buffer2_khr: unsafe {
                        unsafe extern "system" fn cmd_bind_index_buffer2_khr(
                            _command_buffer: CommandBuffer,
                            _buffer: Buffer,
                            _offset: DeviceSize,
                            _size: DeviceSize,
                            _index_type: IndexType,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_bind_index_buffer2_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdBindIndexBuffer2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_bind_index_buffer2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_rendering_area_granularity_khr: unsafe {
                        unsafe extern "system" fn get_rendering_area_granularity_khr(
                            _device: crate::vk::Device,
                            _p_rendering_area_info: *const RenderingAreaInfoKHR<'_>,
                            _p_granularity: *mut Extent2D,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_rendering_area_granularity_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetRenderingAreaGranularityKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_rendering_area_granularity_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_device_image_subresource_layout_khr: unsafe {
                        unsafe extern "system" fn get_device_image_subresource_layout_khr(
                            _device: crate::vk::Device,
                            _p_info: *const DeviceImageSubresourceInfoKHR<'_>,
                            _p_layout: *mut SubresourceLayout2KHR<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_device_image_subresource_layout_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeviceImageSubresourceLayoutKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_device_image_subresource_layout_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_image_subresource_layout2_khr: unsafe {
                        unsafe extern "system" fn get_image_subresource_layout2_khr(
                            _device: crate::vk::Device,
                            _image: Image,
                            _p_subresource: *const ImageSubresource2KHR<'_>,
                            _p_layout: *mut SubresourceLayout2KHR<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_image_subresource_layout2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetImageSubresourceLayout2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_image_subresource_layout2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_ray_tracing_position_fetch"]
    pub mod ray_tracing_position_fetch {
        use super::super::*;
        pub use {
            crate::vk::KHR_RAY_TRACING_POSITION_FETCH_NAME as NAME,
            crate::vk::KHR_RAY_TRACING_POSITION_FETCH_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_cooperative_matrix"]
    pub mod cooperative_matrix {
        use super::super::*;
        pub use {
            crate::vk::KHR_COOPERATIVE_MATRIX_NAME as NAME,
            crate::vk::KHR_COOPERATIVE_MATRIX_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_cooperative_matrix instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_cooperative_matrix instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_cooperative_matrix_properties_khr:
                PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_cooperative_matrix_properties_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_cooperative_matrix_properties_khr(
                            _physical_device: PhysicalDevice,
                            _p_property_count: *mut u32,
                            _p_properties: *mut CooperativeMatrixPropertiesKHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_cooperative_matrix_properties_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_cooperative_matrix_properties_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_video_decode_av1"]
    pub mod video_decode_av1 {
        use super::super::*;
        pub use {
            crate::vk::KHR_VIDEO_DECODE_AV1_NAME as NAME,
            crate::vk::KHR_VIDEO_DECODE_AV1_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_video_maintenance1"]
    pub mod video_maintenance1 {
        use super::super::*;
        pub use {
            crate::vk::KHR_VIDEO_MAINTENANCE1_NAME as NAME,
            crate::vk::KHR_VIDEO_MAINTENANCE1_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_vertex_attribute_divisor"]
    pub mod vertex_attribute_divisor {
        use super::super::*;
        pub use {
            crate::vk::KHR_VERTEX_ATTRIBUTE_DIVISOR_NAME as NAME,
            crate::vk::KHR_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_load_store_op_none"]
    pub mod load_store_op_none {
        use super::super::*;
        pub use {
            crate::vk::KHR_LOAD_STORE_OP_NONE_NAME as NAME,
            crate::vk::KHR_LOAD_STORE_OP_NONE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_shader_float_controls2"]
    pub mod shader_float_controls2 {
        use super::super::*;
        pub use {
            crate::vk::KHR_SHADER_FLOAT_CONTROLS2_NAME as NAME,
            crate::vk::KHR_SHADER_FLOAT_CONTROLS2_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_index_type_uint8"]
    pub mod index_type_uint8 {
        use super::super::*;
        pub use {
            crate::vk::KHR_INDEX_TYPE_UINT8_NAME as NAME,
            crate::vk::KHR_INDEX_TYPE_UINT8_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_line_rasterization"]
    pub mod line_rasterization {
        use super::super::*;
        pub use {
            crate::vk::KHR_LINE_RASTERIZATION_NAME as NAME,
            crate::vk::KHR_LINE_RASTERIZATION_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_line_rasterization device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_line_rasterization device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_set_line_stipple_khr: PFN_vkCmdSetLineStippleKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_set_line_stipple_khr: unsafe {
                        unsafe extern "system" fn cmd_set_line_stipple_khr(
                            _command_buffer: CommandBuffer,
                            _line_stipple_factor: u32,
                            _line_stipple_pattern: u16,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_line_stipple_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLineStippleKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_line_stipple_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_calibrated_timestamps"]
    pub mod calibrated_timestamps {
        use super::super::*;
        pub use {
            crate::vk::KHR_CALIBRATED_TIMESTAMPS_NAME as NAME,
            crate::vk::KHR_CALIBRATED_TIMESTAMPS_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_calibrated_timestamps instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_calibrated_timestamps instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_calibrateable_time_domains_khr:
                PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_calibrateable_time_domains_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_calibrateable_time_domains_khr(
                            _physical_device: PhysicalDevice,
                            _p_time_domain_count: *mut u32,
                            _p_time_domains: *mut TimeDomainKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_calibrateable_time_domains_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceCalibrateableTimeDomainsKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_calibrateable_time_domains_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
        #[doc = "VK_KHR_calibrated_timestamps device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_calibrated_timestamps device-level function pointers"]
        pub struct DeviceFn {
            pub get_calibrated_timestamps_khr: PFN_vkGetCalibratedTimestampsKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_calibrated_timestamps_khr: unsafe {
                        unsafe extern "system" fn get_calibrated_timestamps_khr(
                            _device: crate::vk::Device,
                            _timestamp_count: u32,
                            _p_timestamp_infos: *const CalibratedTimestampInfoKHR<'_>,
                            _p_timestamps: *mut u64,
                            _p_max_deviation: *mut u64,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_calibrated_timestamps_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetCalibratedTimestampsKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_calibrated_timestamps_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_shader_expect_assume"]
    pub mod shader_expect_assume {
        use super::super::*;
        pub use {
            crate::vk::KHR_SHADER_EXPECT_ASSUME_NAME as NAME,
            crate::vk::KHR_SHADER_EXPECT_ASSUME_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_maintenance6"]
    pub mod maintenance6 {
        use super::super::*;
        pub use {
            crate::vk::KHR_MAINTENANCE6_NAME as NAME,
            crate::vk::KHR_MAINTENANCE6_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_maintenance6 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_maintenance6 device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_bind_descriptor_sets2_khr: PFN_vkCmdBindDescriptorSets2KHR,
            pub cmd_push_constants2_khr: PFN_vkCmdPushConstants2KHR,
            pub cmd_push_descriptor_set2_khr: PFN_vkCmdPushDescriptorSet2KHR,
            pub cmd_push_descriptor_set_with_template2_khr:
                PFN_vkCmdPushDescriptorSetWithTemplate2KHR,
            pub cmd_set_descriptor_buffer_offsets2_ext: PFN_vkCmdSetDescriptorBufferOffsets2EXT,
            pub cmd_bind_descriptor_buffer_embedded_samplers2_ext:
                PFN_vkCmdBindDescriptorBufferEmbeddedSamplers2EXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_bind_descriptor_sets2_khr: unsafe {
                        unsafe extern "system" fn cmd_bind_descriptor_sets2_khr(
                            _command_buffer: CommandBuffer,
                            _p_bind_descriptor_sets_info: *const BindDescriptorSetsInfoKHR<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_bind_descriptor_sets2_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdBindDescriptorSets2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_bind_descriptor_sets2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_push_constants2_khr: unsafe {
                        unsafe extern "system" fn cmd_push_constants2_khr(
                            _command_buffer: CommandBuffer,
                            _p_push_constants_info: *const PushConstantsInfoKHR<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_push_constants2_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdPushConstants2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_push_constants2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_push_descriptor_set2_khr: unsafe {
                        unsafe extern "system" fn cmd_push_descriptor_set2_khr(
                            _command_buffer: CommandBuffer,
                            _p_push_descriptor_set_info: *const PushDescriptorSetInfoKHR<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_push_descriptor_set2_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdPushDescriptorSet2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_push_descriptor_set2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_push_descriptor_set_with_template2_khr: unsafe {
                        unsafe extern "system" fn cmd_push_descriptor_set_with_template2_khr(
                            _command_buffer: CommandBuffer,
                            _p_push_descriptor_set_with_template_info : * const PushDescriptorSetWithTemplateInfoKHR < '_ >,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_push_descriptor_set_with_template2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdPushDescriptorSetWithTemplate2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_push_descriptor_set_with_template2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_descriptor_buffer_offsets2_ext: unsafe {
                        unsafe extern "system" fn cmd_set_descriptor_buffer_offsets2_ext(
                            _command_buffer: CommandBuffer,
                            _p_set_descriptor_buffer_offsets_info : * const SetDescriptorBufferOffsetsInfoEXT < '_ >,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_descriptor_buffer_offsets2_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetDescriptorBufferOffsets2EXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_descriptor_buffer_offsets2_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_bind_descriptor_buffer_embedded_samplers2_ext: unsafe {
                        unsafe extern "system" fn cmd_bind_descriptor_buffer_embedded_samplers2_ext(
                            _command_buffer: CommandBuffer,
                            _p_bind_descriptor_buffer_embedded_samplers_info : * const BindDescriptorBufferEmbeddedSamplersInfoEXT < '_ >,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_bind_descriptor_buffer_embedded_samplers2_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdBindDescriptorBufferEmbeddedSamplers2EXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_bind_descriptor_buffer_embedded_samplers2_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
}
#[doc = "Extensions tagged LUNARG"]
pub mod lunarg {
    #[doc = "VK_LUNARG_direct_driver_loading"]
    pub mod direct_driver_loading {
        use super::super::*;
        pub use {
            crate::vk::LUNARG_DIRECT_DRIVER_LOADING_NAME as NAME,
            crate::vk::LUNARG_DIRECT_DRIVER_LOADING_SPEC_VERSION as SPEC_VERSION,
        };
    }
}
#[doc = "Extensions tagged MSFT"]
pub mod msft {
    #[doc = "VK_MSFT_layered_driver"]
    pub mod layered_driver {
        use super::super::*;
        pub use {
            crate::vk::MSFT_LAYERED_DRIVER_NAME as NAME,
            crate::vk::MSFT_LAYERED_DRIVER_SPEC_VERSION as SPEC_VERSION,
        };
    }
}
#[doc = "Extensions tagged MVK"]
pub mod mvk {
    #[doc = "VK_MVK_ios_surface"]
    pub mod ios_surface {
        use super::super::*;
        pub use {
            crate::vk::MVK_IOS_SURFACE_NAME as NAME,
            crate::vk::MVK_IOS_SURFACE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_MVK_ios_surface instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_MVK_ios_surface instance-level function pointers"]
        pub struct InstanceFn {
            pub create_ios_surface_mvk: PFN_vkCreateIOSSurfaceMVK,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_ios_surface_mvk: unsafe {
                        unsafe extern "system" fn create_ios_surface_mvk(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const IOSSurfaceCreateInfoMVK<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_surface: *mut SurfaceKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_ios_surface_mvk)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateIOSSurfaceMVK\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_ios_surface_mvk
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_MVK_macos_surface"]
    pub mod macos_surface {
        use super::super::*;
        pub use {
            crate::vk::MVK_MACOS_SURFACE_NAME as NAME,
            crate::vk::MVK_MACOS_SURFACE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_MVK_macos_surface instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_MVK_macos_surface instance-level function pointers"]
        pub struct InstanceFn {
            pub create_mac_os_surface_mvk: PFN_vkCreateMacOSSurfaceMVK,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_mac_os_surface_mvk: unsafe {
                        unsafe extern "system" fn create_mac_os_surface_mvk(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const MacOSSurfaceCreateInfoMVK<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_surface: *mut SurfaceKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_mac_os_surface_mvk)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateMacOSSurfaceMVK\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_mac_os_surface_mvk
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
}
#[doc = "Extensions tagged NN"]
pub mod nn {
    #[doc = "VK_NN_vi_surface"]
    pub mod vi_surface {
        use super::super::*;
        pub use {
            crate::vk::NN_VI_SURFACE_NAME as NAME,
            crate::vk::NN_VI_SURFACE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_NN_vi_surface instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NN_vi_surface instance-level function pointers"]
        pub struct InstanceFn {
            pub create_vi_surface_nn: PFN_vkCreateViSurfaceNN,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_vi_surface_nn: unsafe {
                        unsafe extern "system" fn create_vi_surface_nn(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const ViSurfaceCreateInfoNN<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_surface: *mut SurfaceKHR,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(create_vi_surface_nn)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateViSurfaceNN\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_vi_surface_nn
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
}
#[doc = "Extensions tagged NV"]
pub mod nv {
    #[doc = "VK_NV_glsl_shader"]
    pub mod glsl_shader {
        use super::super::*;
        pub use {
            crate::vk::NV_GLSL_SHADER_NAME as NAME,
            crate::vk::NV_GLSL_SHADER_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_dedicated_allocation"]
    pub mod dedicated_allocation {
        use super::super::*;
        pub use {
            crate::vk::NV_DEDICATED_ALLOCATION_NAME as NAME,
            crate::vk::NV_DEDICATED_ALLOCATION_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_corner_sampled_image"]
    pub mod corner_sampled_image {
        use super::super::*;
        pub use {
            crate::vk::NV_CORNER_SAMPLED_IMAGE_NAME as NAME,
            crate::vk::NV_CORNER_SAMPLED_IMAGE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_external_memory_capabilities"]
    pub mod external_memory_capabilities {
        use super::super::*;
        pub use {
            crate::vk::NV_EXTERNAL_MEMORY_CAPABILITIES_NAME as NAME,
            crate::vk::NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_NV_external_memory_capabilities instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NV_external_memory_capabilities instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_external_image_format_properties_nv:
                PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_external_image_format_properties_nv: unsafe {
                        unsafe extern "system" fn get_physical_device_external_image_format_properties_nv(
                            _physical_device: PhysicalDevice,
                            _format: Format,
                            _ty: ImageType,
                            _tiling: ImageTiling,
                            _usage: ImageUsageFlags,
                            _flags: ImageCreateFlags,
                            _external_handle_type: ExternalMemoryHandleTypeFlagsNV,
                            _p_external_image_format_properties : * mut ExternalImageFormatPropertiesNV,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_external_image_format_properties_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceExternalImageFormatPropertiesNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_external_image_format_properties_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_NV_external_memory"]
    pub mod external_memory {
        use super::super::*;
        pub use {
            crate::vk::NV_EXTERNAL_MEMORY_NAME as NAME,
            crate::vk::NV_EXTERNAL_MEMORY_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_external_memory_win32"]
    pub mod external_memory_win32 {
        use super::super::*;
        pub use {
            crate::vk::NV_EXTERNAL_MEMORY_WIN32_NAME as NAME,
            crate::vk::NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_NV_external_memory_win32 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NV_external_memory_win32 device-level function pointers"]
        pub struct DeviceFn {
            pub get_memory_win32_handle_nv: PFN_vkGetMemoryWin32HandleNV,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_memory_win32_handle_nv: unsafe {
                        unsafe extern "system" fn get_memory_win32_handle_nv(
                            _device: crate::vk::Device,
                            _memory: DeviceMemory,
                            _handle_type: ExternalMemoryHandleTypeFlagsNV,
                            _p_handle: *mut HANDLE,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_memory_win32_handle_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetMemoryWin32HandleNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_memory_win32_handle_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_NV_win32_keyed_mutex"]
    pub mod win32_keyed_mutex {
        use super::super::*;
        pub use {
            crate::vk::NV_WIN32_KEYED_MUTEX_NAME as NAME,
            crate::vk::NV_WIN32_KEYED_MUTEX_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_clip_space_w_scaling"]
    pub mod clip_space_w_scaling {
        use super::super::*;
        pub use {
            crate::vk::NV_CLIP_SPACE_W_SCALING_NAME as NAME,
            crate::vk::NV_CLIP_SPACE_W_SCALING_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_NV_clip_space_w_scaling device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NV_clip_space_w_scaling device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_set_viewport_w_scaling_nv: PFN_vkCmdSetViewportWScalingNV,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_set_viewport_w_scaling_nv: unsafe {
                        unsafe extern "system" fn cmd_set_viewport_w_scaling_nv(
                            _command_buffer: CommandBuffer,
                            _first_viewport: u32,
                            _viewport_count: u32,
                            _p_viewport_w_scalings: *const ViewportWScalingNV,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_viewport_w_scaling_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetViewportWScalingNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_viewport_w_scaling_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_NV_sample_mask_override_coverage"]
    pub mod sample_mask_override_coverage {
        use super::super::*;
        pub use {
            crate::vk::NV_SAMPLE_MASK_OVERRIDE_COVERAGE_NAME as NAME,
            crate::vk::NV_SAMPLE_MASK_OVERRIDE_COVERAGE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_geometry_shader_passthrough"]
    pub mod geometry_shader_passthrough {
        use super::super::*;
        pub use {
            crate::vk::NV_GEOMETRY_SHADER_PASSTHROUGH_NAME as NAME,
            crate::vk::NV_GEOMETRY_SHADER_PASSTHROUGH_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_viewport_array2"]
    pub mod viewport_array2 {
        use super::super::*;
        pub use {
            crate::vk::NV_VIEWPORT_ARRAY2_NAME as NAME,
            crate::vk::NV_VIEWPORT_ARRAY2_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_viewport_swizzle"]
    pub mod viewport_swizzle {
        use super::super::*;
        pub use {
            crate::vk::NV_VIEWPORT_SWIZZLE_NAME as NAME,
            crate::vk::NV_VIEWPORT_SWIZZLE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_fragment_coverage_to_color"]
    pub mod fragment_coverage_to_color {
        use super::super::*;
        pub use {
            crate::vk::NV_FRAGMENT_COVERAGE_TO_COLOR_NAME as NAME,
            crate::vk::NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_framebuffer_mixed_samples"]
    pub mod framebuffer_mixed_samples {
        use super::super::*;
        pub use {
            crate::vk::NV_FRAMEBUFFER_MIXED_SAMPLES_NAME as NAME,
            crate::vk::NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_fill_rectangle"]
    pub mod fill_rectangle {
        use super::super::*;
        pub use {
            crate::vk::NV_FILL_RECTANGLE_NAME as NAME,
            crate::vk::NV_FILL_RECTANGLE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_shader_sm_builtins"]
    pub mod shader_sm_builtins {
        use super::super::*;
        pub use {
            crate::vk::NV_SHADER_SM_BUILTINS_NAME as NAME,
            crate::vk::NV_SHADER_SM_BUILTINS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_shading_rate_image"]
    pub mod shading_rate_image {
        use super::super::*;
        pub use {
            crate::vk::NV_SHADING_RATE_IMAGE_NAME as NAME,
            crate::vk::NV_SHADING_RATE_IMAGE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_NV_shading_rate_image device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NV_shading_rate_image device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_bind_shading_rate_image_nv: PFN_vkCmdBindShadingRateImageNV,
            pub cmd_set_viewport_shading_rate_palette_nv: PFN_vkCmdSetViewportShadingRatePaletteNV,
            pub cmd_set_coarse_sample_order_nv: PFN_vkCmdSetCoarseSampleOrderNV,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_bind_shading_rate_image_nv: unsafe {
                        unsafe extern "system" fn cmd_bind_shading_rate_image_nv(
                            _command_buffer: CommandBuffer,
                            _image_view: ImageView,
                            _image_layout: ImageLayout,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_bind_shading_rate_image_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdBindShadingRateImageNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_bind_shading_rate_image_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_viewport_shading_rate_palette_nv: unsafe {
                        unsafe extern "system" fn cmd_set_viewport_shading_rate_palette_nv(
                            _command_buffer: CommandBuffer,
                            _first_viewport: u32,
                            _viewport_count: u32,
                            _p_shading_rate_palettes: *const ShadingRatePaletteNV<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_viewport_shading_rate_palette_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetViewportShadingRatePaletteNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_viewport_shading_rate_palette_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coarse_sample_order_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coarse_sample_order_nv(
                            _command_buffer: CommandBuffer,
                            _sample_order_type: CoarseSampleOrderTypeNV,
                            _custom_sample_order_count: u32,
                            _p_custom_sample_orders: *const CoarseSampleOrderCustomNV<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coarse_sample_order_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetCoarseSampleOrderNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coarse_sample_order_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_NV_ray_tracing"]
    pub mod ray_tracing {
        use super::super::*;
        pub use {
            crate::vk::NV_RAY_TRACING_NAME as NAME,
            crate::vk::NV_RAY_TRACING_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_NV_ray_tracing device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NV_ray_tracing device-level function pointers"]
        pub struct DeviceFn {
            pub create_acceleration_structure_nv: PFN_vkCreateAccelerationStructureNV,
            pub destroy_acceleration_structure_nv: PFN_vkDestroyAccelerationStructureNV,
            pub get_acceleration_structure_memory_requirements_nv:
                PFN_vkGetAccelerationStructureMemoryRequirementsNV,
            pub bind_acceleration_structure_memory_nv: PFN_vkBindAccelerationStructureMemoryNV,
            pub cmd_build_acceleration_structure_nv: PFN_vkCmdBuildAccelerationStructureNV,
            pub cmd_copy_acceleration_structure_nv: PFN_vkCmdCopyAccelerationStructureNV,
            pub cmd_trace_rays_nv: PFN_vkCmdTraceRaysNV,
            pub create_ray_tracing_pipelines_nv: PFN_vkCreateRayTracingPipelinesNV,
            pub get_ray_tracing_shader_group_handles_nv: PFN_vkGetRayTracingShaderGroupHandlesKHR,
            pub get_acceleration_structure_handle_nv: PFN_vkGetAccelerationStructureHandleNV,
            pub cmd_write_acceleration_structures_properties_nv:
                PFN_vkCmdWriteAccelerationStructuresPropertiesNV,
            pub compile_deferred_nv: PFN_vkCompileDeferredNV,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_acceleration_structure_nv: unsafe {
                        unsafe extern "system" fn create_acceleration_structure_nv(
                            _device: crate::vk::Device,
                            _p_create_info: *const AccelerationStructureCreateInfoNV<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_acceleration_structure: *mut AccelerationStructureNV,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_acceleration_structure_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCreateAccelerationStructureNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            create_acceleration_structure_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_acceleration_structure_nv: unsafe {
                        unsafe extern "system" fn destroy_acceleration_structure_nv(
                            _device: crate::vk::Device,
                            _acceleration_structure: AccelerationStructureNV,
                            _p_allocator: *const AllocationCallbacks<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(destroy_acceleration_structure_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkDestroyAccelerationStructureNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_acceleration_structure_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_acceleration_structure_memory_requirements_nv: unsafe {
                        unsafe extern "system" fn get_acceleration_structure_memory_requirements_nv(
                            _device: crate::vk::Device,
                            _p_info: *const AccelerationStructureMemoryRequirementsInfoNV<'_>,
                            _p_memory_requirements: *mut MemoryRequirements2KHR<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_acceleration_structure_memory_requirements_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetAccelerationStructureMemoryRequirementsNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_acceleration_structure_memory_requirements_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    bind_acceleration_structure_memory_nv: unsafe {
                        unsafe extern "system" fn bind_acceleration_structure_memory_nv(
                            _device: crate::vk::Device,
                            _bind_info_count: u32,
                            _p_bind_infos: *const BindAccelerationStructureMemoryInfoNV<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(bind_acceleration_structure_memory_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkBindAccelerationStructureMemoryNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            bind_acceleration_structure_memory_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_build_acceleration_structure_nv: unsafe {
                        unsafe extern "system" fn cmd_build_acceleration_structure_nv(
                            _command_buffer: CommandBuffer,
                            _p_info: *const AccelerationStructureInfoNV<'_>,
                            _instance_data: Buffer,
                            _instance_offset: DeviceSize,
                            _update: Bool32,
                            _dst: AccelerationStructureNV,
                            _src: AccelerationStructureNV,
                            _scratch: Buffer,
                            _scratch_offset: DeviceSize,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_build_acceleration_structure_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdBuildAccelerationStructureNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_build_acceleration_structure_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_copy_acceleration_structure_nv: unsafe {
                        unsafe extern "system" fn cmd_copy_acceleration_structure_nv(
                            _command_buffer: CommandBuffer,
                            _dst: AccelerationStructureNV,
                            _src: AccelerationStructureNV,
                            _mode: CopyAccelerationStructureModeKHR,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_copy_acceleration_structure_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdCopyAccelerationStructureNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_copy_acceleration_structure_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_trace_rays_nv: unsafe {
                        unsafe extern "system" fn cmd_trace_rays_nv(
                            _command_buffer: CommandBuffer,
                            _raygen_shader_binding_table_buffer: Buffer,
                            _raygen_shader_binding_offset: DeviceSize,
                            _miss_shader_binding_table_buffer: Buffer,
                            _miss_shader_binding_offset: DeviceSize,
                            _miss_shader_binding_stride: DeviceSize,
                            _hit_shader_binding_table_buffer: Buffer,
                            _hit_shader_binding_offset: DeviceSize,
                            _hit_shader_binding_stride: DeviceSize,
                            _callable_shader_binding_table_buffer: Buffer,
                            _callable_shader_binding_offset: DeviceSize,
                            _callable_shader_binding_stride: DeviceSize,
                            _width: u32,
                            _height: u32,
                            _depth: u32,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_trace_rays_nv)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdTraceRaysNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_trace_rays_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    create_ray_tracing_pipelines_nv: unsafe {
                        unsafe extern "system" fn create_ray_tracing_pipelines_nv(
                            _device: crate::vk::Device,
                            _pipeline_cache: PipelineCache,
                            _create_info_count: u32,
                            _p_create_infos: *const RayTracingPipelineCreateInfoNV<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_pipelines: *mut Pipeline,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_ray_tracing_pipelines_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateRayTracingPipelinesNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_ray_tracing_pipelines_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_ray_tracing_shader_group_handles_nv: unsafe {
                        unsafe extern "system" fn get_ray_tracing_shader_group_handles_nv(
                            _device: crate::vk::Device,
                            _pipeline: Pipeline,
                            _first_group: u32,
                            _group_count: u32,
                            _data_size: usize,
                            _p_data: *mut c_void,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_ray_tracing_shader_group_handles_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetRayTracingShaderGroupHandlesNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_ray_tracing_shader_group_handles_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_acceleration_structure_handle_nv: unsafe {
                        unsafe extern "system" fn get_acceleration_structure_handle_nv(
                            _device: crate::vk::Device,
                            _acceleration_structure: AccelerationStructureNV,
                            _data_size: usize,
                            _p_data: *mut c_void,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_acceleration_structure_handle_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetAccelerationStructureHandleNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_acceleration_structure_handle_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_write_acceleration_structures_properties_nv: unsafe {
                        unsafe extern "system" fn cmd_write_acceleration_structures_properties_nv(
                            _command_buffer: CommandBuffer,
                            _acceleration_structure_count: u32,
                            _p_acceleration_structures: *const AccelerationStructureNV,
                            _query_type: QueryType,
                            _query_pool: QueryPool,
                            _first_query: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_write_acceleration_structures_properties_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdWriteAccelerationStructuresPropertiesNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_write_acceleration_structures_properties_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    compile_deferred_nv: unsafe {
                        unsafe extern "system" fn compile_deferred_nv(
                            _device: crate::vk::Device,
                            _pipeline: Pipeline,
                            _shader: u32,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(compile_deferred_nv)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCompileDeferredNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            compile_deferred_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_NV_representative_fragment_test"]
    pub mod representative_fragment_test {
        use super::super::*;
        pub use {
            crate::vk::NV_REPRESENTATIVE_FRAGMENT_TEST_NAME as NAME,
            crate::vk::NV_REPRESENTATIVE_FRAGMENT_TEST_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_shader_subgroup_partitioned"]
    pub mod shader_subgroup_partitioned {
        use super::super::*;
        pub use {
            crate::vk::NV_SHADER_SUBGROUP_PARTITIONED_NAME as NAME,
            crate::vk::NV_SHADER_SUBGROUP_PARTITIONED_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_compute_shader_derivatives"]
    pub mod compute_shader_derivatives {
        use super::super::*;
        pub use {
            crate::vk::NV_COMPUTE_SHADER_DERIVATIVES_NAME as NAME,
            crate::vk::NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_mesh_shader"]
    pub mod mesh_shader {
        use super::super::*;
        pub use {
            crate::vk::NV_MESH_SHADER_NAME as NAME,
            crate::vk::NV_MESH_SHADER_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_NV_mesh_shader device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NV_mesh_shader device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_draw_mesh_tasks_nv: PFN_vkCmdDrawMeshTasksNV,
            pub cmd_draw_mesh_tasks_indirect_nv: PFN_vkCmdDrawMeshTasksIndirectNV,
            pub cmd_draw_mesh_tasks_indirect_count_nv: PFN_vkCmdDrawMeshTasksIndirectCountNV,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_draw_mesh_tasks_nv: unsafe {
                        unsafe extern "system" fn cmd_draw_mesh_tasks_nv(
                            _command_buffer: CommandBuffer,
                            _task_count: u32,
                            _first_task: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_draw_mesh_tasks_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawMeshTasksNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_draw_mesh_tasks_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_draw_mesh_tasks_indirect_nv: unsafe {
                        unsafe extern "system" fn cmd_draw_mesh_tasks_indirect_nv(
                            _command_buffer: CommandBuffer,
                            _buffer: Buffer,
                            _offset: DeviceSize,
                            _draw_count: u32,
                            _stride: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_draw_mesh_tasks_indirect_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawMeshTasksIndirectNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_draw_mesh_tasks_indirect_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_draw_mesh_tasks_indirect_count_nv: unsafe {
                        unsafe extern "system" fn cmd_draw_mesh_tasks_indirect_count_nv(
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
                                stringify!(cmd_draw_mesh_tasks_indirect_count_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdDrawMeshTasksIndirectCountNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_draw_mesh_tasks_indirect_count_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_NV_fragment_shader_barycentric"]
    pub mod fragment_shader_barycentric {
        use super::super::*;
        pub use {
            crate::vk::NV_FRAGMENT_SHADER_BARYCENTRIC_NAME as NAME,
            crate::vk::NV_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_shader_image_footprint"]
    pub mod shader_image_footprint {
        use super::super::*;
        pub use {
            crate::vk::NV_SHADER_IMAGE_FOOTPRINT_NAME as NAME,
            crate::vk::NV_SHADER_IMAGE_FOOTPRINT_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_scissor_exclusive"]
    pub mod scissor_exclusive {
        use super::super::*;
        pub use {
            crate::vk::NV_SCISSOR_EXCLUSIVE_NAME as NAME,
            crate::vk::NV_SCISSOR_EXCLUSIVE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_NV_scissor_exclusive device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NV_scissor_exclusive device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_set_exclusive_scissor_enable_nv: PFN_vkCmdSetExclusiveScissorEnableNV,
            pub cmd_set_exclusive_scissor_nv: PFN_vkCmdSetExclusiveScissorNV,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_set_exclusive_scissor_enable_nv: unsafe {
                        unsafe extern "system" fn cmd_set_exclusive_scissor_enable_nv(
                            _command_buffer: CommandBuffer,
                            _first_exclusive_scissor: u32,
                            _exclusive_scissor_count: u32,
                            _p_exclusive_scissor_enables: *const Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_exclusive_scissor_enable_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetExclusiveScissorEnableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_exclusive_scissor_enable_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_exclusive_scissor_nv: unsafe {
                        unsafe extern "system" fn cmd_set_exclusive_scissor_nv(
                            _command_buffer: CommandBuffer,
                            _first_exclusive_scissor: u32,
                            _exclusive_scissor_count: u32,
                            _p_exclusive_scissors: *const Rect2D,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_exclusive_scissor_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetExclusiveScissorNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_exclusive_scissor_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_NV_device_diagnostic_checkpoints"]
    pub mod device_diagnostic_checkpoints {
        use super::super::*;
        pub use {
            crate::vk::NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_NAME as NAME,
            crate::vk::NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_NV_device_diagnostic_checkpoints device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NV_device_diagnostic_checkpoints device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_set_checkpoint_nv: PFN_vkCmdSetCheckpointNV,
            pub get_queue_checkpoint_data_nv: PFN_vkGetQueueCheckpointDataNV,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_set_checkpoint_nv: unsafe {
                        unsafe extern "system" fn cmd_set_checkpoint_nv(
                            _command_buffer: CommandBuffer,
                            _p_checkpoint_marker: *const c_void,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_checkpoint_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetCheckpointNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_checkpoint_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_queue_checkpoint_data_nv: unsafe {
                        unsafe extern "system" fn get_queue_checkpoint_data_nv(
                            _queue: Queue,
                            _p_checkpoint_data_count: *mut u32,
                            _p_checkpoint_data: *mut CheckpointDataNV<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_queue_checkpoint_data_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetQueueCheckpointDataNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_queue_checkpoint_data_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_NV_dedicated_allocation_image_aliasing"]
    pub mod dedicated_allocation_image_aliasing {
        use super::super::*;
        pub use {
            crate::vk::NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_NAME as NAME,
            crate::vk::NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_cooperative_matrix"]
    pub mod cooperative_matrix {
        use super::super::*;
        pub use {
            crate::vk::NV_COOPERATIVE_MATRIX_NAME as NAME,
            crate::vk::NV_COOPERATIVE_MATRIX_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_NV_cooperative_matrix instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NV_cooperative_matrix instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_cooperative_matrix_properties_nv:
                PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_cooperative_matrix_properties_nv: unsafe {
                        unsafe extern "system" fn get_physical_device_cooperative_matrix_properties_nv(
                            _physical_device: PhysicalDevice,
                            _p_property_count: *mut u32,
                            _p_properties: *mut CooperativeMatrixPropertiesNV<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_cooperative_matrix_properties_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceCooperativeMatrixPropertiesNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_cooperative_matrix_properties_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_NV_coverage_reduction_mode"]
    pub mod coverage_reduction_mode {
        use super::super::*;
        pub use {
            crate::vk::NV_COVERAGE_REDUCTION_MODE_NAME as NAME,
            crate::vk::NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_NV_coverage_reduction_mode instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NV_coverage_reduction_mode instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_supported_framebuffer_mixed_samples_combinations_nv:
                PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_supported_framebuffer_mixed_samples_combinations_nv: unsafe {
                        unsafe extern "system" fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(
                            _physical_device: PhysicalDevice,
                            _p_combination_count: *mut u32,
                            _p_combinations: *mut FramebufferMixedSamplesCombinationNV<'_>,
                        ) -> Result {
                            panic ! (concat ! ("Unable to load " , stringify ! (get_physical_device_supported_framebuffer_mixed_samples_combinations_nv)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_supported_framebuffer_mixed_samples_combinations_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_NV_device_generated_commands"]
    pub mod device_generated_commands {
        use super::super::*;
        pub use {
            crate::vk::NV_DEVICE_GENERATED_COMMANDS_NAME as NAME,
            crate::vk::NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_NV_device_generated_commands device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NV_device_generated_commands device-level function pointers"]
        pub struct DeviceFn {
            pub get_generated_commands_memory_requirements_nv:
                PFN_vkGetGeneratedCommandsMemoryRequirementsNV,
            pub cmd_preprocess_generated_commands_nv: PFN_vkCmdPreprocessGeneratedCommandsNV,
            pub cmd_execute_generated_commands_nv: PFN_vkCmdExecuteGeneratedCommandsNV,
            pub cmd_bind_pipeline_shader_group_nv: PFN_vkCmdBindPipelineShaderGroupNV,
            pub create_indirect_commands_layout_nv: PFN_vkCreateIndirectCommandsLayoutNV,
            pub destroy_indirect_commands_layout_nv: PFN_vkDestroyIndirectCommandsLayoutNV,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_generated_commands_memory_requirements_nv: unsafe {
                        unsafe extern "system" fn get_generated_commands_memory_requirements_nv(
                            _device: crate::vk::Device,
                            _p_info: *const GeneratedCommandsMemoryRequirementsInfoNV<'_>,
                            _p_memory_requirements: *mut MemoryRequirements2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_generated_commands_memory_requirements_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetGeneratedCommandsMemoryRequirementsNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_generated_commands_memory_requirements_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_preprocess_generated_commands_nv: unsafe {
                        unsafe extern "system" fn cmd_preprocess_generated_commands_nv(
                            _command_buffer: CommandBuffer,
                            _p_generated_commands_info: *const GeneratedCommandsInfoNV<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_preprocess_generated_commands_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdPreprocessGeneratedCommandsNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_preprocess_generated_commands_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_execute_generated_commands_nv: unsafe {
                        unsafe extern "system" fn cmd_execute_generated_commands_nv(
                            _command_buffer: CommandBuffer,
                            _is_preprocessed: Bool32,
                            _p_generated_commands_info: *const GeneratedCommandsInfoNV<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_execute_generated_commands_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdExecuteGeneratedCommandsNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_execute_generated_commands_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_bind_pipeline_shader_group_nv: unsafe {
                        unsafe extern "system" fn cmd_bind_pipeline_shader_group_nv(
                            _command_buffer: CommandBuffer,
                            _pipeline_bind_point: PipelineBindPoint,
                            _pipeline: Pipeline,
                            _group_index: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_bind_pipeline_shader_group_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdBindPipelineShaderGroupNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_bind_pipeline_shader_group_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    create_indirect_commands_layout_nv: unsafe {
                        unsafe extern "system" fn create_indirect_commands_layout_nv(
                            _device: crate::vk::Device,
                            _p_create_info: *const IndirectCommandsLayoutCreateInfoNV<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_indirect_commands_layout: *mut IndirectCommandsLayoutNV,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_indirect_commands_layout_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCreateIndirectCommandsLayoutNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            create_indirect_commands_layout_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_indirect_commands_layout_nv: unsafe {
                        unsafe extern "system" fn destroy_indirect_commands_layout_nv(
                            _device: crate::vk::Device,
                            _indirect_commands_layout: IndirectCommandsLayoutNV,
                            _p_allocator: *const AllocationCallbacks<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(destroy_indirect_commands_layout_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkDestroyIndirectCommandsLayoutNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_indirect_commands_layout_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_NV_inherited_viewport_scissor"]
    pub mod inherited_viewport_scissor {
        use super::super::*;
        pub use {
            crate::vk::NV_INHERITED_VIEWPORT_SCISSOR_NAME as NAME,
            crate::vk::NV_INHERITED_VIEWPORT_SCISSOR_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_present_barrier"]
    pub mod present_barrier {
        use super::super::*;
        pub use {
            crate::vk::NV_PRESENT_BARRIER_NAME as NAME,
            crate::vk::NV_PRESENT_BARRIER_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_device_diagnostics_config"]
    pub mod device_diagnostics_config {
        use super::super::*;
        pub use {
            crate::vk::NV_DEVICE_DIAGNOSTICS_CONFIG_NAME as NAME,
            crate::vk::NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_cuda_kernel_launch"]
    pub mod cuda_kernel_launch {
        use super::super::*;
        pub use {
            crate::vk::NV_CUDA_KERNEL_LAUNCH_NAME as NAME,
            crate::vk::NV_CUDA_KERNEL_LAUNCH_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_NV_cuda_kernel_launch device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NV_cuda_kernel_launch device-level function pointers"]
        pub struct DeviceFn {
            pub create_cuda_module_nv: PFN_vkCreateCudaModuleNV,
            pub get_cuda_module_cache_nv: PFN_vkGetCudaModuleCacheNV,
            pub create_cuda_function_nv: PFN_vkCreateCudaFunctionNV,
            pub destroy_cuda_module_nv: PFN_vkDestroyCudaModuleNV,
            pub destroy_cuda_function_nv: PFN_vkDestroyCudaFunctionNV,
            pub cmd_cuda_launch_kernel_nv: PFN_vkCmdCudaLaunchKernelNV,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_cuda_module_nv: unsafe {
                        unsafe extern "system" fn create_cuda_module_nv(
                            _device: crate::vk::Device,
                            _p_create_info: *const CudaModuleCreateInfoNV<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_module: *mut CudaModuleNV,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_cuda_module_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateCudaModuleNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_cuda_module_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_cuda_module_cache_nv: unsafe {
                        unsafe extern "system" fn get_cuda_module_cache_nv(
                            _device: crate::vk::Device,
                            _module: CudaModuleNV,
                            _p_cache_size: *mut usize,
                            _p_cache_data: *mut c_void,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_cuda_module_cache_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetCudaModuleCacheNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_cuda_module_cache_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    create_cuda_function_nv: unsafe {
                        unsafe extern "system" fn create_cuda_function_nv(
                            _device: crate::vk::Device,
                            _p_create_info: *const CudaFunctionCreateInfoNV<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_function: *mut CudaFunctionNV,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_cuda_function_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateCudaFunctionNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_cuda_function_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_cuda_module_nv: unsafe {
                        unsafe extern "system" fn destroy_cuda_module_nv(
                            _device: crate::vk::Device,
                            _module: CudaModuleNV,
                            _p_allocator: *const AllocationCallbacks<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(destroy_cuda_module_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyCudaModuleNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_cuda_module_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_cuda_function_nv: unsafe {
                        unsafe extern "system" fn destroy_cuda_function_nv(
                            _device: crate::vk::Device,
                            _function: CudaFunctionNV,
                            _p_allocator: *const AllocationCallbacks<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(destroy_cuda_function_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkDestroyCudaFunctionNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_cuda_function_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_cuda_launch_kernel_nv: unsafe {
                        unsafe extern "system" fn cmd_cuda_launch_kernel_nv(
                            _command_buffer: CommandBuffer,
                            _p_launch_info: *const CudaLaunchInfoNV<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_cuda_launch_kernel_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdCudaLaunchKernelNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_cuda_launch_kernel_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_NV_low_latency"]
    pub mod low_latency {
        use super::super::*;
        pub use {
            crate::vk::NV_LOW_LATENCY_NAME as NAME,
            crate::vk::NV_LOW_LATENCY_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_fragment_shading_rate_enums"]
    pub mod fragment_shading_rate_enums {
        use super::super::*;
        pub use {
            crate::vk::NV_FRAGMENT_SHADING_RATE_ENUMS_NAME as NAME,
            crate::vk::NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_NV_fragment_shading_rate_enums device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NV_fragment_shading_rate_enums device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_set_fragment_shading_rate_enum_nv: PFN_vkCmdSetFragmentShadingRateEnumNV,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_set_fragment_shading_rate_enum_nv: unsafe {
                        unsafe extern "system" fn cmd_set_fragment_shading_rate_enum_nv(
                            _command_buffer: CommandBuffer,
                            _shading_rate: FragmentShadingRateNV,
                            _combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2usize],
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_fragment_shading_rate_enum_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetFragmentShadingRateEnumNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_fragment_shading_rate_enum_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_NV_ray_tracing_motion_blur"]
    pub mod ray_tracing_motion_blur {
        use super::super::*;
        pub use {
            crate::vk::NV_RAY_TRACING_MOTION_BLUR_NAME as NAME,
            crate::vk::NV_RAY_TRACING_MOTION_BLUR_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_acquire_winrt_display"]
    pub mod acquire_winrt_display {
        use super::super::*;
        pub use {
            crate::vk::NV_ACQUIRE_WINRT_DISPLAY_NAME as NAME,
            crate::vk::NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_NV_acquire_winrt_display instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NV_acquire_winrt_display instance-level function pointers"]
        pub struct InstanceFn {
            pub acquire_winrt_display_nv: PFN_vkAcquireWinrtDisplayNV,
            pub get_winrt_display_nv: PFN_vkGetWinrtDisplayNV,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    acquire_winrt_display_nv: unsafe {
                        unsafe extern "system" fn acquire_winrt_display_nv(
                            _physical_device: PhysicalDevice,
                            _display: DisplayKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(acquire_winrt_display_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkAcquireWinrtDisplayNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            acquire_winrt_display_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_winrt_display_nv: unsafe {
                        unsafe extern "system" fn get_winrt_display_nv(
                            _physical_device: PhysicalDevice,
                            _device_relative_id: u32,
                            _p_display: *mut DisplayKHR,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(get_winrt_display_nv)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetWinrtDisplayNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_winrt_display_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_NV_external_memory_rdma"]
    pub mod external_memory_rdma {
        use super::super::*;
        pub use {
            crate::vk::NV_EXTERNAL_MEMORY_RDMA_NAME as NAME,
            crate::vk::NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_NV_external_memory_rdma device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NV_external_memory_rdma device-level function pointers"]
        pub struct DeviceFn {
            pub get_memory_remote_address_nv: PFN_vkGetMemoryRemoteAddressNV,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_memory_remote_address_nv: unsafe {
                        unsafe extern "system" fn get_memory_remote_address_nv(
                            _device: crate::vk::Device,
                            _p_memory_get_remote_address_info: *const MemoryGetRemoteAddressInfoNV<
                                '_,
                            >,
                            _p_address: *mut RemoteAddressNV,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_memory_remote_address_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetMemoryRemoteAddressNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_memory_remote_address_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_NV_displacement_micromap"]
    pub mod displacement_micromap {
        use super::super::*;
        pub use {
            crate::vk::NV_DISPLACEMENT_MICROMAP_NAME as NAME,
            crate::vk::NV_DISPLACEMENT_MICROMAP_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_copy_memory_indirect"]
    pub mod copy_memory_indirect {
        use super::super::*;
        pub use {
            crate::vk::NV_COPY_MEMORY_INDIRECT_NAME as NAME,
            crate::vk::NV_COPY_MEMORY_INDIRECT_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_NV_copy_memory_indirect device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NV_copy_memory_indirect device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_copy_memory_indirect_nv: PFN_vkCmdCopyMemoryIndirectNV,
            pub cmd_copy_memory_to_image_indirect_nv: PFN_vkCmdCopyMemoryToImageIndirectNV,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_copy_memory_indirect_nv: unsafe {
                        unsafe extern "system" fn cmd_copy_memory_indirect_nv(
                            _command_buffer: CommandBuffer,
                            _copy_buffer_address: DeviceAddress,
                            _copy_count: u32,
                            _stride: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_copy_memory_indirect_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyMemoryIndirectNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_copy_memory_indirect_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_copy_memory_to_image_indirect_nv: unsafe {
                        unsafe extern "system" fn cmd_copy_memory_to_image_indirect_nv(
                            _command_buffer: CommandBuffer,
                            _copy_buffer_address: DeviceAddress,
                            _copy_count: u32,
                            _stride: u32,
                            _dst_image: Image,
                            _dst_image_layout: ImageLayout,
                            _p_image_subresources: *const ImageSubresourceLayers,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_copy_memory_to_image_indirect_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdCopyMemoryToImageIndirectNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_copy_memory_to_image_indirect_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_NV_memory_decompression"]
    pub mod memory_decompression {
        use super::super::*;
        pub use {
            crate::vk::NV_MEMORY_DECOMPRESSION_NAME as NAME,
            crate::vk::NV_MEMORY_DECOMPRESSION_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_NV_memory_decompression device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NV_memory_decompression device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_decompress_memory_nv: PFN_vkCmdDecompressMemoryNV,
            pub cmd_decompress_memory_indirect_count_nv: PFN_vkCmdDecompressMemoryIndirectCountNV,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_decompress_memory_nv: unsafe {
                        unsafe extern "system" fn cmd_decompress_memory_nv(
                            _command_buffer: CommandBuffer,
                            _decompress_region_count: u32,
                            _p_decompress_memory_regions: *const DecompressMemoryRegionNV,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_decompress_memory_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdDecompressMemoryNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_decompress_memory_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_decompress_memory_indirect_count_nv: unsafe {
                        unsafe extern "system" fn cmd_decompress_memory_indirect_count_nv(
                            _command_buffer: CommandBuffer,
                            _indirect_commands_address: DeviceAddress,
                            _indirect_commands_count_address: DeviceAddress,
                            _stride: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_decompress_memory_indirect_count_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdDecompressMemoryIndirectCountNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_decompress_memory_indirect_count_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_NV_device_generated_commands_compute"]
    pub mod device_generated_commands_compute {
        use super::super::*;
        pub use {
            crate::vk::NV_DEVICE_GENERATED_COMMANDS_COMPUTE_NAME as NAME,
            crate::vk::NV_DEVICE_GENERATED_COMMANDS_COMPUTE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_NV_device_generated_commands_compute device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NV_device_generated_commands_compute device-level function pointers"]
        pub struct DeviceFn {
            pub get_pipeline_indirect_memory_requirements_nv:
                PFN_vkGetPipelineIndirectMemoryRequirementsNV,
            pub cmd_update_pipeline_indirect_buffer_nv: PFN_vkCmdUpdatePipelineIndirectBufferNV,
            pub get_pipeline_indirect_device_address_nv: PFN_vkGetPipelineIndirectDeviceAddressNV,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_pipeline_indirect_memory_requirements_nv: unsafe {
                        unsafe extern "system" fn get_pipeline_indirect_memory_requirements_nv(
                            _device: crate::vk::Device,
                            _p_create_info: *const ComputePipelineCreateInfo<'_>,
                            _p_memory_requirements: *mut MemoryRequirements2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_pipeline_indirect_memory_requirements_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPipelineIndirectMemoryRequirementsNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_pipeline_indirect_memory_requirements_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_update_pipeline_indirect_buffer_nv: unsafe {
                        unsafe extern "system" fn cmd_update_pipeline_indirect_buffer_nv(
                            _command_buffer: CommandBuffer,
                            _pipeline_bind_point: PipelineBindPoint,
                            _pipeline: Pipeline,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_update_pipeline_indirect_buffer_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdUpdatePipelineIndirectBufferNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_update_pipeline_indirect_buffer_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_pipeline_indirect_device_address_nv: unsafe {
                        unsafe extern "system" fn get_pipeline_indirect_device_address_nv(
                            _device: crate::vk::Device,
                            _p_info: *const PipelineIndirectDeviceAddressInfoNV<'_>,
                        ) -> DeviceAddress {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_pipeline_indirect_device_address_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPipelineIndirectDeviceAddressNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_pipeline_indirect_device_address_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_NV_linear_color_attachment"]
    pub mod linear_color_attachment {
        use super::super::*;
        pub use {
            crate::vk::NV_LINEAR_COLOR_ATTACHMENT_NAME as NAME,
            crate::vk::NV_LINEAR_COLOR_ATTACHMENT_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_optical_flow"]
    pub mod optical_flow {
        use super::super::*;
        pub use {
            crate::vk::NV_OPTICAL_FLOW_NAME as NAME,
            crate::vk::NV_OPTICAL_FLOW_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_NV_optical_flow instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NV_optical_flow instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_optical_flow_image_formats_nv:
                PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_optical_flow_image_formats_nv: unsafe {
                        unsafe extern "system" fn get_physical_device_optical_flow_image_formats_nv(
                            _physical_device: PhysicalDevice,
                            _p_optical_flow_image_format_info: *const OpticalFlowImageFormatInfoNV<
                                '_,
                            >,
                            _p_format_count: *mut u32,
                            _p_image_format_properties: *mut OpticalFlowImageFormatPropertiesNV<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_optical_flow_image_formats_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceOpticalFlowImageFormatsNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_optical_flow_image_formats_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
        #[doc = "VK_NV_optical_flow device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NV_optical_flow device-level function pointers"]
        pub struct DeviceFn {
            pub create_optical_flow_session_nv: PFN_vkCreateOpticalFlowSessionNV,
            pub destroy_optical_flow_session_nv: PFN_vkDestroyOpticalFlowSessionNV,
            pub bind_optical_flow_session_image_nv: PFN_vkBindOpticalFlowSessionImageNV,
            pub cmd_optical_flow_execute_nv: PFN_vkCmdOpticalFlowExecuteNV,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_optical_flow_session_nv: unsafe {
                        unsafe extern "system" fn create_optical_flow_session_nv(
                            _device: crate::vk::Device,
                            _p_create_info: *const OpticalFlowSessionCreateInfoNV<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_session: *mut OpticalFlowSessionNV,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_optical_flow_session_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateOpticalFlowSessionNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_optical_flow_session_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_optical_flow_session_nv: unsafe {
                        unsafe extern "system" fn destroy_optical_flow_session_nv(
                            _device: crate::vk::Device,
                            _session: OpticalFlowSessionNV,
                            _p_allocator: *const AllocationCallbacks<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(destroy_optical_flow_session_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkDestroyOpticalFlowSessionNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_optical_flow_session_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    bind_optical_flow_session_image_nv: unsafe {
                        unsafe extern "system" fn bind_optical_flow_session_image_nv(
                            _device: crate::vk::Device,
                            _session: OpticalFlowSessionNV,
                            _binding_point: OpticalFlowSessionBindingPointNV,
                            _view: ImageView,
                            _layout: ImageLayout,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(bind_optical_flow_session_image_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkBindOpticalFlowSessionImageNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            bind_optical_flow_session_image_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_optical_flow_execute_nv: unsafe {
                        unsafe extern "system" fn cmd_optical_flow_execute_nv(
                            _command_buffer: CommandBuffer,
                            _session: OpticalFlowSessionNV,
                            _p_execute_info: *const OpticalFlowExecuteInfoNV<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_optical_flow_execute_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdOpticalFlowExecuteNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_optical_flow_execute_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_NV_ray_tracing_invocation_reorder"]
    pub mod ray_tracing_invocation_reorder {
        use super::super::*;
        pub use {
            crate::vk::NV_RAY_TRACING_INVOCATION_REORDER_NAME as NAME,
            crate::vk::NV_RAY_TRACING_INVOCATION_REORDER_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_extended_sparse_address_space"]
    pub mod extended_sparse_address_space {
        use super::super::*;
        pub use {
            crate::vk::NV_EXTENDED_SPARSE_ADDRESS_SPACE_NAME as NAME,
            crate::vk::NV_EXTENDED_SPARSE_ADDRESS_SPACE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_low_latency2"]
    pub mod low_latency2 {
        use super::super::*;
        pub use {
            crate::vk::NV_LOW_LATENCY2_NAME as NAME,
            crate::vk::NV_LOW_LATENCY2_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_NV_low_latency2 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NV_low_latency2 device-level function pointers"]
        pub struct DeviceFn {
            pub set_latency_sleep_mode_nv: PFN_vkSetLatencySleepModeNV,
            pub latency_sleep_nv: PFN_vkLatencySleepNV,
            pub set_latency_marker_nv: PFN_vkSetLatencyMarkerNV,
            pub get_latency_timings_nv: PFN_vkGetLatencyTimingsNV,
            pub queue_notify_out_of_band_nv: PFN_vkQueueNotifyOutOfBandNV,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    set_latency_sleep_mode_nv: unsafe {
                        unsafe extern "system" fn set_latency_sleep_mode_nv(
                            _device: crate::vk::Device,
                            _swapchain: SwapchainKHR,
                            _p_sleep_mode_info: *const LatencySleepModeInfoNV<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(set_latency_sleep_mode_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkSetLatencySleepModeNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            set_latency_sleep_mode_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    latency_sleep_nv: unsafe {
                        unsafe extern "system" fn latency_sleep_nv(
                            _device: crate::vk::Device,
                            _swapchain: SwapchainKHR,
                            _p_sleep_info: *const LatencySleepInfoNV<'_>,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(latency_sleep_nv)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkLatencySleepNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            latency_sleep_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    set_latency_marker_nv: unsafe {
                        unsafe extern "system" fn set_latency_marker_nv(
                            _device: crate::vk::Device,
                            _swapchain: SwapchainKHR,
                            _p_latency_marker_info: *const SetLatencyMarkerInfoNV<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(set_latency_marker_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkSetLatencyMarkerNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            set_latency_marker_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_latency_timings_nv: unsafe {
                        unsafe extern "system" fn get_latency_timings_nv(
                            _device: crate::vk::Device,
                            _swapchain: SwapchainKHR,
                            _p_latency_marker_info: *mut GetLatencyMarkerInfoNV<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_latency_timings_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetLatencyTimingsNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_latency_timings_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    queue_notify_out_of_band_nv: unsafe {
                        unsafe extern "system" fn queue_notify_out_of_band_nv(
                            _queue: Queue,
                            _p_queue_type_info: *const OutOfBandQueueTypeInfoNV<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(queue_notify_out_of_band_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkQueueNotifyOutOfBandNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            queue_notify_out_of_band_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_NV_per_stage_descriptor_set"]
    pub mod per_stage_descriptor_set {
        use super::super::*;
        pub use {
            crate::vk::NV_PER_STAGE_DESCRIPTOR_SET_NAME as NAME,
            crate::vk::NV_PER_STAGE_DESCRIPTOR_SET_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_descriptor_pool_overallocation"]
    pub mod descriptor_pool_overallocation {
        use super::super::*;
        pub use {
            crate::vk::NV_DESCRIPTOR_POOL_OVERALLOCATION_NAME as NAME,
            crate::vk::NV_DESCRIPTOR_POOL_OVERALLOCATION_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_raw_access_chains"]
    pub mod raw_access_chains {
        use super::super::*;
        pub use {
            crate::vk::NV_RAW_ACCESS_CHAINS_NAME as NAME,
            crate::vk::NV_RAW_ACCESS_CHAINS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_shader_atomic_float16_vector"]
    pub mod shader_atomic_float16_vector {
        use super::super::*;
        pub use {
            crate::vk::NV_SHADER_ATOMIC_FLOAT16_VECTOR_NAME as NAME,
            crate::vk::NV_SHADER_ATOMIC_FLOAT16_VECTOR_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_NV_ray_tracing_validation"]
    pub mod ray_tracing_validation {
        use super::super::*;
        pub use {
            crate::vk::NV_RAY_TRACING_VALIDATION_NAME as NAME,
            crate::vk::NV_RAY_TRACING_VALIDATION_SPEC_VERSION as SPEC_VERSION,
        };
    }
}
#[doc = "Extensions tagged NVX"]
pub mod nvx {
    #[doc = "VK_NVX_binary_import"]
    pub mod binary_import {
        use super::super::*;
        pub use {
            crate::vk::NVX_BINARY_IMPORT_NAME as NAME,
            crate::vk::NVX_BINARY_IMPORT_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_NVX_binary_import device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NVX_binary_import device-level function pointers"]
        pub struct DeviceFn {
            pub create_cu_module_nvx: PFN_vkCreateCuModuleNVX,
            pub create_cu_function_nvx: PFN_vkCreateCuFunctionNVX,
            pub destroy_cu_module_nvx: PFN_vkDestroyCuModuleNVX,
            pub destroy_cu_function_nvx: PFN_vkDestroyCuFunctionNVX,
            pub cmd_cu_launch_kernel_nvx: PFN_vkCmdCuLaunchKernelNVX,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_cu_module_nvx: unsafe {
                        unsafe extern "system" fn create_cu_module_nvx(
                            _device: crate::vk::Device,
                            _p_create_info: *const CuModuleCreateInfoNVX<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_module: *mut CuModuleNVX,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(create_cu_module_nvx)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateCuModuleNVX\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_cu_module_nvx
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    create_cu_function_nvx: unsafe {
                        unsafe extern "system" fn create_cu_function_nvx(
                            _device: crate::vk::Device,
                            _p_create_info: *const CuFunctionCreateInfoNVX<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_function: *mut CuFunctionNVX,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_cu_function_nvx)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateCuFunctionNVX\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_cu_function_nvx
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_cu_module_nvx: unsafe {
                        unsafe extern "system" fn destroy_cu_module_nvx(
                            _device: crate::vk::Device,
                            _module: CuModuleNVX,
                            _p_allocator: *const AllocationCallbacks<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(destroy_cu_module_nvx)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyCuModuleNVX\0");
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_cu_module_nvx
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_cu_function_nvx: unsafe {
                        unsafe extern "system" fn destroy_cu_function_nvx(
                            _device: crate::vk::Device,
                            _function: CuFunctionNVX,
                            _p_allocator: *const AllocationCallbacks<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(destroy_cu_function_nvx)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkDestroyCuFunctionNVX\0");
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_cu_function_nvx
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_cu_launch_kernel_nvx: unsafe {
                        unsafe extern "system" fn cmd_cu_launch_kernel_nvx(
                            _command_buffer: CommandBuffer,
                            _p_launch_info: *const CuLaunchInfoNVX<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_cu_launch_kernel_nvx)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdCuLaunchKernelNVX\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_cu_launch_kernel_nvx
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_NVX_image_view_handle"]
    pub mod image_view_handle {
        use super::super::*;
        pub use {
            crate::vk::NVX_IMAGE_VIEW_HANDLE_NAME as NAME,
            crate::vk::NVX_IMAGE_VIEW_HANDLE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_NVX_image_view_handle device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_NVX_image_view_handle device-level function pointers"]
        pub struct DeviceFn {
            pub get_image_view_handle_nvx: PFN_vkGetImageViewHandleNVX,
            pub get_image_view_address_nvx: PFN_vkGetImageViewAddressNVX,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_image_view_handle_nvx: unsafe {
                        unsafe extern "system" fn get_image_view_handle_nvx(
                            _device: crate::vk::Device,
                            _p_info: *const ImageViewHandleInfoNVX<'_>,
                        ) -> u32 {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_image_view_handle_nvx)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetImageViewHandleNVX\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_image_view_handle_nvx
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_image_view_address_nvx: unsafe {
                        unsafe extern "system" fn get_image_view_address_nvx(
                            _device: crate::vk::Device,
                            _image_view: ImageView,
                            _p_properties: *mut ImageViewAddressPropertiesNVX<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_image_view_address_nvx)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetImageViewAddressNVX\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_image_view_address_nvx
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_NVX_multiview_per_view_attributes"]
    pub mod multiview_per_view_attributes {
        use super::super::*;
        pub use {
            crate::vk::NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_NAME as NAME,
            crate::vk::NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION as SPEC_VERSION,
        };
    }
}
#[doc = "Extensions tagged QCOM"]
pub mod qcom {
    #[doc = "VK_QCOM_render_pass_shader_resolve"]
    pub mod render_pass_shader_resolve {
        use super::super::*;
        pub use {
            crate::vk::QCOM_RENDER_PASS_SHADER_RESOLVE_NAME as NAME,
            crate::vk::QCOM_RENDER_PASS_SHADER_RESOLVE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_QCOM_render_pass_transform"]
    pub mod render_pass_transform {
        use super::super::*;
        pub use {
            crate::vk::QCOM_RENDER_PASS_TRANSFORM_NAME as NAME,
            crate::vk::QCOM_RENDER_PASS_TRANSFORM_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_QCOM_render_pass_store_ops"]
    pub mod render_pass_store_ops {
        use super::super::*;
        pub use {
            crate::vk::QCOM_RENDER_PASS_STORE_OPS_NAME as NAME,
            crate::vk::QCOM_RENDER_PASS_STORE_OPS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_QCOM_rotated_copy_commands"]
    pub mod rotated_copy_commands {
        use super::super::*;
        pub use {
            crate::vk::QCOM_ROTATED_COPY_COMMANDS_NAME as NAME,
            crate::vk::QCOM_ROTATED_COPY_COMMANDS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_QCOM_fragment_density_map_offset"]
    pub mod fragment_density_map_offset {
        use super::super::*;
        pub use {
            crate::vk::QCOM_FRAGMENT_DENSITY_MAP_OFFSET_NAME as NAME,
            crate::vk::QCOM_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_QCOM_image_processing"]
    pub mod image_processing {
        use super::super::*;
        pub use {
            crate::vk::QCOM_IMAGE_PROCESSING_NAME as NAME,
            crate::vk::QCOM_IMAGE_PROCESSING_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_QCOM_tile_properties"]
    pub mod tile_properties {
        use super::super::*;
        pub use {
            crate::vk::QCOM_TILE_PROPERTIES_NAME as NAME,
            crate::vk::QCOM_TILE_PROPERTIES_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_QCOM_tile_properties device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_QCOM_tile_properties device-level function pointers"]
        pub struct DeviceFn {
            pub get_framebuffer_tile_properties_qcom: PFN_vkGetFramebufferTilePropertiesQCOM,
            pub get_dynamic_rendering_tile_properties_qcom:
                PFN_vkGetDynamicRenderingTilePropertiesQCOM,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_framebuffer_tile_properties_qcom: unsafe {
                        unsafe extern "system" fn get_framebuffer_tile_properties_qcom(
                            _device: crate::vk::Device,
                            _framebuffer: Framebuffer,
                            _p_properties_count: *mut u32,
                            _p_properties: *mut TilePropertiesQCOM<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_framebuffer_tile_properties_qcom)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetFramebufferTilePropertiesQCOM\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_framebuffer_tile_properties_qcom
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_dynamic_rendering_tile_properties_qcom: unsafe {
                        unsafe extern "system" fn get_dynamic_rendering_tile_properties_qcom(
                            _device: crate::vk::Device,
                            _p_rendering_info: *const RenderingInfo<'_>,
                            _p_properties: *mut TilePropertiesQCOM<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_dynamic_rendering_tile_properties_qcom)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDynamicRenderingTilePropertiesQCOM\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_dynamic_rendering_tile_properties_qcom
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_QCOM_multiview_per_view_viewports"]
    pub mod multiview_per_view_viewports {
        use super::super::*;
        pub use {
            crate::vk::QCOM_MULTIVIEW_PER_VIEW_VIEWPORTS_NAME as NAME,
            crate::vk::QCOM_MULTIVIEW_PER_VIEW_VIEWPORTS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_QCOM_multiview_per_view_render_areas"]
    pub mod multiview_per_view_render_areas {
        use super::super::*;
        pub use {
            crate::vk::QCOM_MULTIVIEW_PER_VIEW_RENDER_AREAS_NAME as NAME,
            crate::vk::QCOM_MULTIVIEW_PER_VIEW_RENDER_AREAS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_QCOM_image_processing2"]
    pub mod image_processing2 {
        use super::super::*;
        pub use {
            crate::vk::QCOM_IMAGE_PROCESSING2_NAME as NAME,
            crate::vk::QCOM_IMAGE_PROCESSING2_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_QCOM_filter_cubic_weights"]
    pub mod filter_cubic_weights {
        use super::super::*;
        pub use {
            crate::vk::QCOM_FILTER_CUBIC_WEIGHTS_NAME as NAME,
            crate::vk::QCOM_FILTER_CUBIC_WEIGHTS_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_QCOM_ycbcr_degamma"]
    pub mod ycbcr_degamma {
        use super::super::*;
        pub use {
            crate::vk::QCOM_YCBCR_DEGAMMA_NAME as NAME,
            crate::vk::QCOM_YCBCR_DEGAMMA_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_QCOM_filter_cubic_clamp"]
    pub mod filter_cubic_clamp {
        use super::super::*;
        pub use {
            crate::vk::QCOM_FILTER_CUBIC_CLAMP_NAME as NAME,
            crate::vk::QCOM_FILTER_CUBIC_CLAMP_SPEC_VERSION as SPEC_VERSION,
        };
    }
}
#[doc = "Extensions tagged QNX"]
pub mod qnx {
    #[doc = "VK_QNX_screen_surface"]
    pub mod screen_surface {
        use super::super::*;
        pub use {
            crate::vk::QNX_SCREEN_SURFACE_NAME as NAME,
            crate::vk::QNX_SCREEN_SURFACE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_QNX_screen_surface instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle();
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_QNX_screen_surface instance-level function pointers"]
        pub struct InstanceFn {
            pub create_screen_surface_qnx: PFN_vkCreateScreenSurfaceQNX,
            pub get_physical_device_screen_presentation_support_qnx:
                PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_screen_surface_qnx: unsafe {
                        unsafe extern "system" fn create_screen_surface_qnx(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const ScreenSurfaceCreateInfoQNX<'_>,
                            _p_allocator: *const AllocationCallbacks<'_>,
                            _p_surface: *mut SurfaceKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_screen_surface_qnx)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateScreenSurfaceQNX\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_screen_surface_qnx
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_screen_presentation_support_qnx: unsafe {
                        unsafe extern "system" fn get_physical_device_screen_presentation_support_qnx(
                            _physical_device: PhysicalDevice,
                            _queue_family_index: u32,
                            _window: *mut _screen_window,
                        ) -> Bool32 {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_screen_presentation_support_qnx)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceScreenPresentationSupportQNX\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_screen_presentation_support_qnx
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_QNX_external_memory_screen_buffer"]
    pub mod external_memory_screen_buffer {
        use super::super::*;
        pub use {
            crate::vk::QNX_EXTERNAL_MEMORY_SCREEN_BUFFER_NAME as NAME,
            crate::vk::QNX_EXTERNAL_MEMORY_SCREEN_BUFFER_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_QNX_external_memory_screen_buffer device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_QNX_external_memory_screen_buffer device-level function pointers"]
        pub struct DeviceFn {
            pub get_screen_buffer_properties_qnx: PFN_vkGetScreenBufferPropertiesQNX,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_screen_buffer_properties_qnx: unsafe {
                        unsafe extern "system" fn get_screen_buffer_properties_qnx(
                            _device: crate::vk::Device,
                            _buffer: *const _screen_buffer,
                            _p_properties: *mut ScreenBufferPropertiesQNX<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_screen_buffer_properties_qnx)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetScreenBufferPropertiesQNX\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_screen_buffer_properties_qnx
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
}
#[doc = "Extensions tagged SEC"]
pub mod sec {
    #[doc = "VK_SEC_amigo_profiling"]
    pub mod amigo_profiling {
        use super::super::*;
        pub use {
            crate::vk::SEC_AMIGO_PROFILING_NAME as NAME,
            crate::vk::SEC_AMIGO_PROFILING_SPEC_VERSION as SPEC_VERSION,
        };
    }
}
#[doc = "Extensions tagged VALVE"]
pub mod valve {
    #[doc = "VK_VALVE_mutable_descriptor_type"]
    pub mod mutable_descriptor_type {
        use super::super::*;
        pub use {
            crate::vk::VALVE_MUTABLE_DESCRIPTOR_TYPE_NAME as NAME,
            crate::vk::VALVE_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_VALVE_descriptor_set_host_mapping"]
    pub mod descriptor_set_host_mapping {
        use super::super::*;
        pub use {
            crate::vk::VALVE_DESCRIPTOR_SET_HOST_MAPPING_NAME as NAME,
            crate::vk::VALVE_DESCRIPTOR_SET_HOST_MAPPING_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_VALVE_descriptor_set_host_mapping device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle();
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_VALVE_descriptor_set_host_mapping device-level function pointers"]
        pub struct DeviceFn {
            pub get_descriptor_set_layout_host_mapping_info_valve:
                PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE,
            pub get_descriptor_set_host_mapping_valve: PFN_vkGetDescriptorSetHostMappingVALVE,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_descriptor_set_layout_host_mapping_info_valve: unsafe {
                        unsafe extern "system" fn get_descriptor_set_layout_host_mapping_info_valve(
                            _device: crate::vk::Device,
                            _p_binding_reference: *const DescriptorSetBindingReferenceVALVE<'_>,
                            _p_host_mapping: *mut DescriptorSetLayoutHostMappingInfoVALVE<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_descriptor_set_layout_host_mapping_info_valve)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDescriptorSetLayoutHostMappingInfoVALVE\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_descriptor_set_layout_host_mapping_info_valve
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_descriptor_set_host_mapping_valve: unsafe {
                        unsafe extern "system" fn get_descriptor_set_host_mapping_valve(
                            _device: crate::vk::Device,
                            _descriptor_set: DescriptorSet,
                            _pp_data: *mut *mut c_void,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_descriptor_set_host_mapping_valve)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDescriptorSetHostMappingVALVE\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_descriptor_set_host_mapping_valve
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
}
