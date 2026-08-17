//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_dynamic_rendering_local_read.html>

use crate::vk;

impl crate::khr::dynamic_rendering_local_read::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRenderingAttachmentLocationsKHR.html>
    #[inline]
    pub unsafe fn cmd_set_rendering_attachment_locations(
        &self,
        command_buffer: vk::CommandBuffer,
        location_info: &vk::RenderingAttachmentLocationInfoKHR<'_>,
    ) {
        (self.fp.cmd_set_rendering_attachment_locations_khr)(command_buffer, location_info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRenderingInputAttachmentIndicesKHR.html>
    #[inline]
    pub unsafe fn cmd_set_rendering_input_attachment_indices(
        &self,
        command_buffer: vk::CommandBuffer,
        location_info: &vk::RenderingInputAttachmentIndexInfoKHR<'_>,
    ) {
        (self.fp.cmd_set_rendering_input_attachment_indices_khr)(command_buffer, location_info)
    }
}
