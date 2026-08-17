//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_ray_tracing.html>

use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use alloc::vec::Vec;
use core::mem;

impl crate::nv::ray_tracing::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureNV.html>
    #[inline]
    pub unsafe fn create_acceleration_structure(
        &self,
        create_info: &vk::AccelerationStructureCreateInfoNV<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::AccelerationStructureNV> {
        let mut accel_struct = mem::MaybeUninit::uninit();
        (self.fp.create_acceleration_structure_nv)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            accel_struct.as_mut_ptr(),
        )
        .assume_init_on_success(accel_struct)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureNV.html>
    #[inline]
    pub unsafe fn destroy_acceleration_structure(
        &self,
        accel_struct: vk::AccelerationStructureNV,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.fp.destroy_acceleration_structure_nv)(
            self.handle,
            accel_struct,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsNV.html>
    #[inline]
    pub unsafe fn get_acceleration_structure_memory_requirements(
        &self,
        info: &vk::AccelerationStructureMemoryRequirementsInfoNV<'_>,
    ) -> vk::MemoryRequirements2KHR<'_> {
        let mut requirements = Default::default();
        (self.fp.get_acceleration_structure_memory_requirements_nv)(
            self.handle,
            info,
            &mut requirements,
        );
        requirements
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindAccelerationStructureMemoryNV.html>
    #[inline]
    pub unsafe fn bind_acceleration_structure_memory(
        &self,
        bind_info: &[vk::BindAccelerationStructureMemoryInfoNV<'_>],
    ) -> VkResult<()> {
        (self.fp.bind_acceleration_structure_memory_nv)(
            self.handle,
            bind_info.len() as u32,
            bind_info.as_ptr(),
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructureNV.html>
    #[inline]
    pub unsafe fn cmd_build_acceleration_structure(
        &self,
        command_buffer: vk::CommandBuffer,
        info: &vk::AccelerationStructureInfoNV<'_>,
        instance_data: vk::Buffer,
        instance_offset: vk::DeviceSize,
        update: bool,
        dst: vk::AccelerationStructureNV,
        src: vk::AccelerationStructureNV,
        scratch: vk::Buffer,
        scratch_offset: vk::DeviceSize,
    ) {
        (self.fp.cmd_build_acceleration_structure_nv)(
            command_buffer,
            info,
            instance_data,
            instance_offset,
            if update { vk::TRUE } else { vk::FALSE },
            dst,
            src,
            scratch,
            scratch_offset,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureNV.html>
    #[inline]
    pub unsafe fn cmd_copy_acceleration_structure(
        &self,
        command_buffer: vk::CommandBuffer,
        dst: vk::AccelerationStructureNV,
        src: vk::AccelerationStructureNV,
        mode: vk::CopyAccelerationStructureModeNV,
    ) {
        (self.fp.cmd_copy_acceleration_structure_nv)(command_buffer, dst, src, mode);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysNV.html>
    #[inline]
    pub unsafe fn cmd_trace_rays(
        &self,
        command_buffer: vk::CommandBuffer,
        raygen_shader_binding_table_buffer: vk::Buffer,
        raygen_shader_binding_offset: vk::DeviceSize,
        miss_shader_binding_table_buffer: vk::Buffer,
        miss_shader_binding_offset: vk::DeviceSize,
        miss_shader_binding_stride: vk::DeviceSize,
        hit_shader_binding_table_buffer: vk::Buffer,
        hit_shader_binding_offset: vk::DeviceSize,
        hit_shader_binding_stride: vk::DeviceSize,
        callable_shader_binding_table_buffer: vk::Buffer,
        callable_shader_binding_offset: vk::DeviceSize,
        callable_shader_binding_stride: vk::DeviceSize,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        (self.fp.cmd_trace_rays_nv)(
            command_buffer,
            raygen_shader_binding_table_buffer,
            raygen_shader_binding_offset,
            miss_shader_binding_table_buffer,
            miss_shader_binding_offset,
            miss_shader_binding_stride,
            hit_shader_binding_table_buffer,
            hit_shader_binding_offset,
            hit_shader_binding_stride,
            callable_shader_binding_table_buffer,
            callable_shader_binding_offset,
            callable_shader_binding_stride,
            width,
            height,
            depth,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesNV.html>
    ///
    /// Pipelines are created and returned as described for [Multiple Pipeline Creation].
    ///
    /// [Multiple Pipeline Creation]: https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-multiple
    #[inline]
    pub unsafe fn create_ray_tracing_pipelines(
        &self,
        pipeline_cache: vk::PipelineCache,
        create_infos: &[vk::RayTracingPipelineCreateInfoNV<'_>],
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> Result<Vec<vk::Pipeline>, (Vec<vk::Pipeline>, vk::Result)> {
        let mut pipelines = Vec::with_capacity(create_infos.len());
        let err_code = (self.fp.create_ray_tracing_pipelines_nv)(
            self.handle,
            pipeline_cache,
            create_infos.len() as u32,
            create_infos.as_ptr(),
            allocation_callbacks.as_raw_ptr(),
            pipelines.as_mut_ptr(),
        );
        pipelines.set_len(create_infos.len());
        match err_code {
            vk::Result::SUCCESS => Ok(pipelines),
            _ => Err((pipelines, err_code)),
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupHandlesNV.html>
    #[inline]
    pub unsafe fn get_ray_tracing_shader_group_handles(
        &self,
        pipeline: vk::Pipeline,
        first_group: u32,
        group_count: u32,
        data: &mut [u8],
    ) -> VkResult<()> {
        (self.fp.get_ray_tracing_shader_group_handles_nv)(
            self.handle,
            pipeline,
            first_group,
            group_count,
            data.len(),
            data.as_mut_ptr().cast(),
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureHandleNV.html>
    #[inline]
    pub unsafe fn get_acceleration_structure_handle(
        &self,
        accel_struct: vk::AccelerationStructureNV,
    ) -> VkResult<u64> {
        let mut handle = mem::MaybeUninit::<u64>::uninit();
        (self.fp.get_acceleration_structure_handle_nv)(
            self.handle,
            accel_struct,
            mem::size_of_val(&handle),
            handle.as_mut_ptr().cast(),
        )
        .assume_init_on_success(handle)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesNV.html>
    #[inline]
    pub unsafe fn cmd_write_acceleration_structures_properties(
        &self,
        command_buffer: vk::CommandBuffer,
        structures: &[vk::AccelerationStructureNV],
        query_type: vk::QueryType,
        query_pool: vk::QueryPool,
        first_query: u32,
    ) {
        (self.fp.cmd_write_acceleration_structures_properties_nv)(
            command_buffer,
            structures.len() as u32,
            structures.as_ptr(),
            query_type,
            query_pool,
            first_query,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCompileDeferredNV.html>
    #[inline]
    pub unsafe fn compile_deferred(&self, pipeline: vk::Pipeline, shader: u32) -> VkResult<()> {
        (self.fp.compile_deferred_nv)(self.handle, pipeline, shader).result()
    }
}
