//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_swapchain_maintenance1.html>

use crate::prelude::*;
use crate::vk;

impl crate::ext::swapchain_maintenance1::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleaseSwapchainImagesEXT.html>
    #[inline]
    pub unsafe fn release_swapchain_images(
        &self,
        release_info: &vk::ReleaseSwapchainImagesInfoEXT<'_>,
    ) -> VkResult<()> {
        (self.fp.release_swapchain_images_ext)(self.handle, release_info).result()
    }
}
