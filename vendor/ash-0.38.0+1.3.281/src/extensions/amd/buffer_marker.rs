//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_buffer_marker.html>

use crate::vk;

impl crate::amd::buffer_marker::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarkerAMD.html>
    #[inline]
    pub unsafe fn cmd_write_buffer_marker(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_stage: vk::PipelineStageFlags,
        dst_buffer: vk::Buffer,
        dst_offset: vk::DeviceSize,
        marker: u32,
    ) {
        (self.fp.cmd_write_buffer_marker_amd)(
            command_buffer,
            pipeline_stage,
            dst_buffer,
            dst_offset,
            marker,
        )
    }
}
