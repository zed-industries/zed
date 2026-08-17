//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_device_generated_commands_compute.html>

use crate::vk;

impl crate::nv::device_generated_commands_compute::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineIndirectMemoryRequirementsNV.html>
    #[inline]
    pub unsafe fn get_pipeline_indirect_memory_requirements(
        &self,
        create_info: &vk::ComputePipelineCreateInfo<'_>,
        memory_requirements: &mut vk::MemoryRequirements2<'_>,
    ) {
        (self.fp.get_pipeline_indirect_memory_requirements_nv)(
            self.handle,
            create_info,
            memory_requirements,
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdUpdatePipelineIndirectBufferNV.html>
    #[inline]
    pub unsafe fn cmd_update_pipeline_indirect_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        pipeline: vk::Pipeline,
    ) {
        (self.fp.cmd_update_pipeline_indirect_buffer_nv)(
            command_buffer,
            pipeline_bind_point,
            pipeline,
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineIndirectDeviceAddressNV.html>
    #[inline]
    pub unsafe fn get_pipeline_indirect_device_address(
        &self,
        info: &vk::PipelineIndirectDeviceAddressInfoNV<'_>,
    ) -> vk::DeviceAddress {
        (self.fp.get_pipeline_indirect_device_address_nv)(self.handle, info)
    }
}
