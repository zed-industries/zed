//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_acquire_drm_display.html>

use crate::prelude::*;
use crate::vk;
use core::mem;

impl crate::ext::acquire_drm_display::Instance {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireDrmDisplayEXT.html>
    #[inline]
    pub unsafe fn acquire_drm_display(
        &self,
        physical_device: vk::PhysicalDevice,
        drm_fd: i32,
        display: vk::DisplayKHR,
    ) -> VkResult<()> {
        (self.fp.acquire_drm_display_ext)(physical_device, drm_fd, display).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDrmDisplayEXT.html>
    #[inline]
    pub unsafe fn get_drm_display(
        &self,
        physical_device: vk::PhysicalDevice,
        drm_fd: i32,
        connector_id: u32,
    ) -> VkResult<vk::DisplayKHR> {
        let mut display = mem::MaybeUninit::uninit();
        (self.fp.get_drm_display_ext)(physical_device, drm_fd, connector_id, display.as_mut_ptr())
            .assume_init_on_success(display)
    }
}
