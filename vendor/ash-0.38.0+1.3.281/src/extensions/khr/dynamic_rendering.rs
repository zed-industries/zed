//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_dynamic_rendering.html>

use crate::vk;

impl crate::khr::dynamic_rendering::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderingKHR.html>
    #[inline]
    pub unsafe fn cmd_begin_rendering(
        &self,
        command_buffer: vk::CommandBuffer,
        rendering_info: &vk::RenderingInfoKHR<'_>,
    ) {
        (self.fp.cmd_begin_rendering_khr)(command_buffer, rendering_info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderingKHR.html>
    #[inline]
    pub unsafe fn cmd_end_rendering(&self, command_buffer: vk::CommandBuffer) {
        (self.fp.cmd_end_rendering_khr)(command_buffer)
    }
}
