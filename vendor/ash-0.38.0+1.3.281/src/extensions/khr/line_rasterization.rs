//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_line_rasterization.html>

use crate::vk;

impl crate::khr::line_rasterization::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineStippleKHR.html>
    #[inline]
    pub unsafe fn cmd_set_line_stipple(
        &self,
        command_buffer: vk::CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        (self.fp.cmd_set_line_stipple_khr)(
            command_buffer,
            line_stipple_factor,
            line_stipple_pattern,
        )
    }
}
