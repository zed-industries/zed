//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_maintenance1.html>

use crate::vk;

impl crate::khr::maintenance1::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkTrimCommandPoolKHR.html>
    #[inline]
    pub unsafe fn trim_command_pool(
        &self,
        command_pool: vk::CommandPool,
        flags: vk::CommandPoolTrimFlagsKHR,
    ) {
        (self.fp.trim_command_pool_khr)(self.handle, command_pool, flags);
    }
}
