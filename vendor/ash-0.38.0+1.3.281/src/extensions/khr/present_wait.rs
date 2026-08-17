//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_present_wait.html>

use crate::prelude::*;
use crate::vk;

impl crate::khr::present_wait::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWaitForPresentKHR.html>
    #[inline]
    pub unsafe fn wait_for_present(
        &self,
        swapchain: vk::SwapchainKHR,
        present_id: u64,
        timeout: u64,
    ) -> VkResult<()> {
        (self.fp.wait_for_present_khr)(self.handle, swapchain, present_id, timeout).result()
    }
}
